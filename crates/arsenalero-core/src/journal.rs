use ring::digest::{SHA256, digest};

/// Closed event vocabulary persisted by the append-only case journal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JournalEventType {
    CaseInitialized,
    StageEntered,
    ResourceIssued,
    ResourceAttested,
    ReceiptStale,
    ResourceModifiedPostAttestation,
    SkillDigestChanged,
    CaseReconciled,
}

impl JournalEventType {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CaseInitialized => "case_initialized",
            Self::StageEntered => "stage_entered",
            Self::ResourceIssued => "resource_issued",
            Self::ResourceAttested => "resource_attested",
            Self::ReceiptStale => "receipt_stale",
            Self::ResourceModifiedPostAttestation => "resource_modified_post_attestation",
            Self::SkillDigestChanged => "skill_digest_changed",
            Self::CaseReconciled => "case_reconciled",
        }
    }
}

/// One immutable, hash-linked journal record.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JournalEvent {
    sequence: u64,
    event_type: JournalEventType,
    timestamp: String,
    payload: String,
    previous_digest: Option<String>,
    event_digest: String,
}

impl JournalEvent {
    pub const fn sequence(&self) -> u64 {
        self.sequence
    }
    pub const fn event_type(&self) -> JournalEventType {
        self.event_type
    }
    pub fn previous_digest(&self) -> Option<&str> {
        self.previous_digest.as_deref()
    }
    pub fn event_digest(&self) -> &str {
        &self.event_digest
    }
}

/// In-memory append-only journal; persistence is deliberately kept at the adapter boundary.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct JournalWriter {
    events: Vec<JournalEvent>,
    terminal_digest: Option<String>,
}

impl JournalWriter {
    pub fn append(
        &mut self,
        event_type: JournalEventType,
        timestamp: impl Into<String>,
        payload: impl Into<String>,
    ) -> &JournalEvent {
        self.terminal_digest = None;
        let sequence = self.events.len() as u64 + 1;
        let timestamp = timestamp.into();
        let payload = payload.into();
        let previous_digest = self.events.last().map(|event| event.event_digest.clone());
        let event_digest = event_digest(
            sequence,
            event_type,
            &timestamp,
            &payload,
            previous_digest.as_deref(),
        );
        self.events.push(JournalEvent {
            sequence,
            event_type,
            timestamp,
            payload,
            previous_digest,
            event_digest,
        });
        self.events.last().expect("event was appended")
    }

    pub fn events(&self) -> &[JournalEvent] {
        &self.events
    }

    pub fn close(&mut self) {
        let digest = self.events.last().map(JournalEvent::event_digest);
        self.terminal_digest =
            digest.map(|digest| terminal_digest(self.events.len() as u64, digest));
    }

    /// Validates ordering and every hash-link, failing closed on any mismatch.
    pub fn is_valid(&self) -> bool {
        if self.events.is_empty() {
            return false;
        }

        let mut previous_digest = None;
        let valid = self.events.iter().enumerate().all(|(index, event)| {
            let sequence = index as u64 + 1;
            let valid = event.sequence == sequence
                && event.previous_digest.as_deref() == previous_digest.as_deref()
                && event.event_digest
                    == event_digest(
                        sequence,
                        event.event_type,
                        &event.timestamp,
                        &event.payload,
                        previous_digest.as_deref(),
                    );
            previous_digest = Some(event.event_digest.clone());
            valid
        });

        valid
            && self.terminal_digest.as_deref()
                == previous_digest
                    .as_deref()
                    .map(|digest| terminal_digest(self.events.len() as u64, digest))
                    .as_deref()
    }
}

fn terminal_digest(event_count: u64, last_event_digest: &str) -> String {
    let mut canonical = b"arsenalero-journal-terminal-v1".to_vec();
    canonical.extend_from_slice(&event_count.to_be_bytes());
    append_framed(&mut canonical, last_event_digest.as_bytes());
    let hash = digest(&SHA256, &canonical);
    format!("sha256:{}", lowercase_hex(hash.as_ref()))
}

fn event_digest(
    sequence: u64,
    event_type: JournalEventType,
    timestamp: &str,
    payload: &str,
    previous_digest: Option<&str>,
) -> String {
    let mut canonical = Vec::new();
    canonical.extend_from_slice(b"arsenalero-journal-event-v1");
    canonical.extend_from_slice(&sequence.to_be_bytes());
    append_framed(&mut canonical, event_type.as_str().as_bytes());
    append_framed(&mut canonical, timestamp.as_bytes());
    append_framed(&mut canonical, payload.as_bytes());
    match previous_digest {
        Some(digest) => {
            canonical.push(1);
            append_framed(&mut canonical, digest.as_bytes());
        }
        None => canonical.push(0),
    }
    format!(
        "sha256:{}",
        lowercase_hex(digest(&SHA256, &canonical).as_ref())
    )
}

/// Appends a length-delimited field so no two field sequences share an encoding.
fn append_framed(output: &mut Vec<u8>, value: &[u8]) {
    output.extend_from_slice(&(value.len() as u64).to_be_bytes());
    output.extend_from_slice(value);
}

fn lowercase_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }
    output
}

#[cfg(test)]
mod tests {
    use super::{JournalEventType, JournalWriter};

    #[test]
    fn empty_or_tail_truncated_journal_is_invalid() {
        assert!(!JournalWriter::default().is_valid());

        let mut journal = JournalWriter::default();
        journal.append(JournalEventType::CaseInitialized, "t0", "payload");
        journal.append(JournalEventType::ResourceIssued, "t1", "payload");
        journal.close();
        journal.events.pop();

        assert!(!journal.is_valid());
    }
}

use std::{
    collections::BTreeSet,
    fs::File,
    io::{self, Read, Seek, SeekFrom},
    path::Path,
    time::SystemTime,
};

use ring::digest::{Context, SHA256};

use crate::{
    ArsenalCase, ArsenalError, AttainedEvidenceLevel, CanonicalResourcePath, EvidenceContract,
    ReceiptId, ResourceId,
};

const MAX_RESOURCES_PER_ISSUE: usize = 4;
const MAX_RESOURCES_PER_CASE: usize = 128;
const MAX_ISSUE_CONTENT_BYTES: usize = 256 * 1024;
const MAX_ISSUE_CONTENT_BYTES_PER_CASE: usize = 2 * 1024 * 1024;
const MAX_SKILL_MD_BYTES: usize = 512 * 1024;
const MAX_ATTESTATIONS_PER_REQUEST: usize = 16;
const DIGEST_BUFFER_BYTES: usize = 8 * 1024;

/// The immutable input necessary to issue a resource receipt.
#[derive(Debug)]
pub struct ResourceIssueRequest {
    pub resource_id: ResourceId,
    pub resource: CanonicalResourcePath,
    pub purpose: String,
    pub evidence_contract: EvidenceContract,
}

/// Capability information disclosed at issue time, without attained evidence.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IssuedResource {
    pub resource_id: ResourceId,
    pub digest: String,
    pub purpose: String,
    pub content: String,
    evidence_contract: EvidenceContract,
}

/// A one-time case-bound receipt for an issued resource.
#[derive(Debug, PartialEq, Eq)]
pub struct ResourceReceipt {
    id: ReceiptId,
    case_id: crate::CaseId,
    resource_digest: String,
    skill_digest: String,
    issued_at: SystemTime,
    resource: IssuedResource,
    resource_handle: CanonicalResourcePath,
}

impl Clone for ResourceReceipt {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            case_id: self.case_id.clone(),
            resource_digest: self.resource_digest.clone(),
            skill_digest: self.skill_digest.clone(),
            issued_at: self.issued_at,
            resource: self.resource.clone(),
            resource_handle: CanonicalResourcePath::from_retained(
                self.resource_handle.as_path().to_owned(),
                self.resource_handle
                    .try_clone_file()
                    .expect("validated resource handle should remain clonable"),
            ),
        }
    }
}

impl ResourceReceipt {
    pub fn id(&self) -> &ReceiptId {
        &self.id
    }
    pub fn case_id(&self) -> &crate::CaseId {
        &self.case_id
    }
    pub fn resource_digest(&self) -> &str {
        &self.resource_digest
    }
    pub fn skill_digest(&self) -> &str {
        &self.skill_digest
    }
    pub fn resource(&self) -> &IssuedResource {
        &self.resource
    }
}

impl IssuedResource {
    pub fn evidence_contract(&self) -> &EvidenceContract {
        &self.evidence_contract
    }
}

/// A named, agent-supplied artifact attribution.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EvidenceReference {
    pub reference: String,
}

impl EvidenceReference {
    pub fn artifact(reference: impl Into<String>) -> Self {
        Self {
            reference: reference.into(),
        }
    }
}

/// An attestation request for a single receipt.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AttestationRequest {
    pub receipt_id: ReceiptId,
    pub usage: String,
    pub evidence: Vec<EvidenceReference>,
}

impl AttestationRequest {
    pub fn used(receipt_id: ReceiptId, usage: impl Into<String>) -> Self {
        Self {
            receipt_id,
            usage: usage.into(),
            evidence: Vec::new(),
        }
    }
}

/// A persisted attestation result with server-computed evidence level.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Attestation {
    pub receipt_id: ReceiptId,
    pub usage: String,
    pub evidence: Vec<EvidenceReference>,
    pub attained_evidence_level: AttainedEvidenceLevel,
}

/// Issues up to four resources, binding each receipt to the current case and digests.
pub fn issue_resources(
    case: &ArsenalCase,
    resources: &[ResourceIssueRequest],
) -> Result<Vec<ResourceReceipt>, ArsenalError> {
    if resources.len() > MAX_RESOURCES_PER_ISSUE {
        return Err(ArsenalError::ResourceBatchLimit);
    }

    validate_skill_digest(case)?;
    let mut issued_resources = case.issued_resources()?;
    if issued_resources.ids.len() + resources.len() > MAX_RESOURCES_PER_CASE {
        return Err(ArsenalError::ResourceBatchLimit);
    }
    let mut requested_resources = BTreeSet::new();
    for resource in resources {
        if !resource.resource.as_path().starts_with(
            case.skill_path()
                .parent()
                .ok_or(ArsenalError::ResourcePathEscape)?,
        ) {
            return Err(ArsenalError::ResourcePathEscape);
        }
        if !requested_resources.insert(resource.resource_id.clone())
            || issued_resources.ids.contains(&resource.resource_id)
        {
            return Err(ArsenalError::ResourceAlreadyIssued);
        }
    }
    let mut total_content_bytes = 0;
    let mut receipts = Vec::with_capacity(resources.len());
    for resource in resources {
        let receipt = issue_one(
            case,
            resource,
            MAX_ISSUE_CONTENT_BYTES - total_content_bytes,
        )?;
        total_content_bytes += receipt.resource.content.len();
        receipts.push(receipt);
    }
    if issued_resources.content_bytes + total_content_bytes > MAX_ISSUE_CONTENT_BYTES_PER_CASE {
        return Err(ArsenalError::ResourceLimitExceeded);
    }
    issued_resources.ids.extend(requested_resources);
    issued_resources.content_bytes += total_content_bytes;
    Ok(receipts)
}

/// Revalidates digests and records up to sixteen attestations for the supplied receipts.
pub fn attest_resources(
    case: &ArsenalCase,
    receipts: &[ResourceReceipt],
    attestations: &[AttestationRequest],
) -> Result<Vec<Attestation>, ArsenalError> {
    if attestations.len() > MAX_ATTESTATIONS_PER_REQUEST {
        return Err(ArsenalError::ResourceBatchLimit);
    }
    validate_skill_digest(case)?;

    let mut consumed_receipts = case.consumed_receipts()?;
    let mut requested_receipts = BTreeSet::new();
    let results = attestations
        .iter()
        .map(|attestation| {
            if !requested_receipts.insert(attestation.receipt_id.clone())
                || consumed_receipts.contains(&attestation.receipt_id)
            {
                return Err(ArsenalError::ReceiptUnknown);
            }
            let receipt = receipts
                .iter()
                .find(|receipt| receipt.id == attestation.receipt_id)
                .ok_or(ArsenalError::ReceiptUnknown)?;
            attest_one(case, receipt, attestation)
        })
        .collect::<Result<Vec<_>, _>>()?;
    consumed_receipts.extend(requested_receipts);
    Ok(results)
}

fn issue_one(
    case: &ArsenalCase,
    request: &ResourceIssueRequest,
    content_limit: usize,
) -> Result<ResourceReceipt, ArsenalError> {
    let (content, resource_digest) = read_content_and_digest(
        request
            .resource
            .try_clone_file()
            .map_err(|_| ArsenalError::ResourceReferenceBroken)?,
        content_limit,
    )?;
    Ok(ResourceReceipt {
        id: ReceiptId::now_v7(),
        case_id: case.id().clone(),
        resource_digest: resource_digest.clone(),
        skill_digest: case.skill_digest().to_owned(),
        issued_at: SystemTime::now(),
        resource: IssuedResource {
            resource_id: request.resource_id.clone(),
            digest: resource_digest,
            purpose: request.purpose.clone(),
            content,
            evidence_contract: request.evidence_contract.clone(),
        },
        resource_handle: request
            .resource
            .try_clone_file()
            .map_err(|_| ArsenalError::ResourceReferenceBroken)
            .map(|file| {
                CanonicalResourcePath::from_retained(request.resource.as_path().to_owned(), file)
            })?,
    })
}

fn attest_one(
    case: &ArsenalCase,
    receipt: &ResourceReceipt,
    request: &AttestationRequest,
) -> Result<Attestation, ArsenalError> {
    if receipt.case_id != *case.id() {
        return Err(ArsenalError::ReceiptCaseMismatch);
    }
    if receipt.skill_digest != case.skill_digest() {
        return Err(ArsenalError::ReceiptStale);
    }
    if request.usage.trim().is_empty() {
        return Err(ArsenalError::AttestationEmpty);
    }
    if request
        .evidence
        .iter()
        .any(|evidence| evidence.reference.trim().is_empty())
    {
        return Err(ArsenalError::EvidenceReferenceInvalid);
    }
    let actual_digest = digest_file(receipt.resource_handle.as_path())
        .map_err(|_| ArsenalError::ResourceReferenceBroken)?;
    if actual_digest != receipt.resource_digest {
        return Err(ArsenalError::ReceiptStale);
    }

    let attained_evidence_level = if request.evidence.is_empty() {
        AttainedEvidenceLevel::Attestation
    } else {
        AttainedEvidenceLevel::ArtifactReference
    };
    if !receipt
        .resource
        .evidence_contract
        .supported_levels
        .contains(&attained_evidence_level)
        || evidence_rank(attained_evidence_level)
            < evidence_rank(receipt.resource.evidence_contract.minimum)
    {
        return Err(ArsenalError::EvidenceReferenceInvalid);
    }
    Ok(Attestation {
        receipt_id: receipt.id.clone(),
        usage: request.usage.clone(),
        evidence: request.evidence.clone(),
        attained_evidence_level,
    })
}

fn evidence_rank(level: AttainedEvidenceLevel) -> u8 {
    match level {
        AttainedEvidenceLevel::Attestation => 0,
        AttainedEvidenceLevel::ArtifactReference => 1,
    }
}

fn validate_skill_digest(case: &ArsenalCase) -> Result<(), ArsenalError> {
    let actual_digest = digest_skill_file(case.skill_path())?;
    if actual_digest != case.skill_digest() {
        return Err(ArsenalError::SkillDigestChanged);
    }
    Ok(())
}

/// Computes a SHA-256 digest incrementally without loading the full file into memory.
pub(crate) fn digest_file(path: &Path) -> io::Result<String> {
    digest_open_file(File::open(path)?)
}

pub(crate) fn digest_skill_file(path: &Path) -> Result<String, ArsenalError> {
    digest_open_file_limited(File::open(path).map_err(|_| ArsenalError::SkillMdMissing)?)
}

fn digest_open_file_limited(mut file: File) -> Result<String, ArsenalError> {
    file.seek(SeekFrom::Start(0))
        .map_err(|_| ArsenalError::SkillMdMissing)?;
    let mut context = Context::new(&SHA256);
    let mut buffer = [0_u8; DIGEST_BUFFER_BYTES];
    let mut total = 0;
    loop {
        let read = file
            .read(&mut buffer)
            .map_err(|_| ArsenalError::SkillMdMissing)?;
        if read == 0 {
            break;
        }
        total += read;
        if total > MAX_SKILL_MD_BYTES {
            return Err(ArsenalError::SkillMdTooLarge);
        }
        context.update(&buffer[..read]);
    }
    Ok(format!(
        "sha256:{}",
        lowercase_hex(context.finish().as_ref())
    ))
}

fn digest_open_file(mut file: File) -> io::Result<String> {
    file.seek(SeekFrom::Start(0))?;
    let mut context = Context::new(&SHA256);
    let mut buffer = [0_u8; DIGEST_BUFFER_BYTES];
    loop {
        let read = file.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        context.update(&buffer[..read]);
    }
    Ok(format!(
        "sha256:{}",
        lowercase_hex(context.finish().as_ref())
    ))
}

fn read_content_and_digest(file: File, limit: usize) -> Result<(String, String), ArsenalError> {
    let mut file = file;
    file.seek(SeekFrom::Start(0))
        .map_err(|_| ArsenalError::ResourceReferenceBroken)?;
    let mut bytes = Vec::new();
    let mut context = Context::new(&SHA256);
    let mut buffer = [0_u8; DIGEST_BUFFER_BYTES];
    loop {
        let read = file
            .read(&mut buffer)
            .map_err(|_| ArsenalError::ResourceReferenceBroken)?;
        if read == 0 {
            break;
        }
        if bytes.len().saturating_add(read) > limit {
            return Err(ArsenalError::ResourceLimitExceeded);
        }
        context.update(&buffer[..read]);
        bytes.extend_from_slice(&buffer[..read]);
    }
    let content = String::from_utf8(bytes).map_err(|_| ArsenalError::ResourceReferenceBroken)?;
    Ok((
        content,
        format!("sha256:{}", lowercase_hex(context.finish().as_ref())),
    ))
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

use std::{future, sync::Arc};

use rmcp::{
    ServerHandler, ServiceExt,
    model::{
        CallToolRequestParams, CallToolResult, Implementation, ListToolsResult, ServerCapabilities,
        ServerInfo, Tool,
    },
    service::{RequestContext, RoleServer},
    transport::stdio,
};

use crate::{
    schema::*,
    tools::{self, ServerState},
};

/// MCP adapter for the five Arsenalero resource-lifecycle tools.
#[derive(Clone, Debug)]
pub struct ArsenaleroServer {
    state: Arc<ServerState>,
}

impl Default for ArsenaleroServer {
    fn default() -> Self {
        Self::new()
    }
}

impl ArsenaleroServer {
    pub fn new() -> Self {
        Self {
            state: Arc::new(ServerState::empty()),
        }
    }

    pub fn from_environment() -> Result<Self, arsenalero_core::ArsenalError> {
        Ok(Self {
            state: Arc::new(ServerState::from_environment()?),
        })
    }
}

impl ServerHandler for ArsenaleroServer {
    fn get_info(&self) -> ServerInfo {
        let mut info = ServerInfo::new(ServerCapabilities::builder().enable_tools().build());
        info.server_info = Implementation::new("arsenalero", env!("CARGO_PKG_VERSION"));
        info.instructions = Some("Manages resource custody after a skill is active. It exposes only arsenal_init, arsenal_stage, arsenal_issue, arsenal_attest, and arsenal_reconcile.".to_owned());
        info
    }

    fn list_tools(
        &self,
        _: Option<rmcp::model::PaginatedRequestParams>,
        _: RequestContext<RoleServer>,
    ) -> impl Future<Output = Result<ListToolsResult, rmcp::ErrorData>>
    + rmcp::service::MaybeSendFuture
    + '_ {
        future::ready(Ok(ListToolsResult {
            tools: tool_definitions(),
            ..Default::default()
        }))
    }

    fn get_tool(&self, name: &str) -> Option<Tool> {
        tool_definitions()
            .into_iter()
            .find(|tool| tool.name == name)
    }

    fn call_tool(
        &self,
        request: CallToolRequestParams,
        _: RequestContext<RoleServer>,
    ) -> impl Future<Output = Result<CallToolResult, rmcp::ErrorData>>
    + rmcp::service::MaybeSendFuture
    + '_ {
        future::ready(Ok(dispatch(&self.state, request)))
    }
}

fn dispatch(state: &ServerState, request: CallToolRequestParams) -> CallToolResult {
    let arguments = serde_json::Value::Object(request.arguments.unwrap_or_default());
    macro_rules! decode { ($type:ty, $handler:path) => { match serde_json::from_value::<$type>(arguments) { Ok(input) => $handler(state, input), Err(_) => CallToolResult::structured_error(serde_json::json!({"error_code": "INVALID_ARGUMENTS", "message": "Arguments do not satisfy the tool schema."})), } }; }
    match request.name.as_ref() {
        "arsenal_init" => decode!(InitInput, tools::init),
        "arsenal_stage" => decode!(StageInput, tools::stage),
        "arsenal_issue" => decode!(IssueInput, tools::issue),
        "arsenal_attest" => decode!(AttestInput, tools::attest),
        "arsenal_reconcile" => decode!(ReconcileInput, tools::reconcile_case),
        _ => CallToolResult::structured_error(
            serde_json::json!({"error_code": "TOOL_UNKNOWN", "message": "Tool is not exposed by Arsenalero."}),
        ),
    }
}

fn tool_definitions() -> Vec<Tool> {
    vec![
        Tool::new("arsenal_init", "Purpose: open a resource-custody case and inventory the active skill. Precondition: the skill is already active and its absolute root is supplied. Non-goal: skill activation or workflow control. When to call: once after activation. Usual next step: arsenal_stage.", serde_json::Map::new()).with_input_schema::<InitInput>().with_output_schema::<InitOutput>(),
        Tool::new("arsenal_stage", "Purpose: identify resources relevant to a declared workflow stage. Precondition: a ready case exists. Non-goal: changing the frozen REQUIRED set. When to call: when entering a stage. Usual next step: arsenal_issue.", serde_json::Map::new()).with_input_schema::<StageInput>().with_output_schema::<StageOutput>(),
        Tool::new("arsenal_issue", "Purpose: issue up to four inventoried resources with case-bound receipts. Precondition: a ready case and known resource IDs. Non-goal: chunking, automatic reissue, or evidence attainment. When to call: before applying referenced resources. Usual next step: arsenal_attest.", serde_json::Map::new()).with_input_schema::<IssueInput>().with_output_schema::<IssueOutput>(),
        Tool::new("arsenal_attest", "Purpose: record use of issued receipts and server-computed evidence level. Precondition: same-case current receipts and non-empty usage. Non-goal: external artifact verification. When to call: after using issued resources. Usual next step: arsenal_reconcile.", serde_json::Map::new()).with_input_schema::<AttestInput>().with_output_schema::<AttestOutput>(),
        Tool::new("arsenal_reconcile", "Purpose: return an idempotent final custody summary. Precondition: a case exists. Non-goal: judging the primary task result or external verification. When to call: before completing the workflow. Usual next step: surface the summary to the user.", serde_json::Map::new()).with_input_schema::<ReconcileInput>().with_output_schema::<ReconcileOutput>(),
    ]
}

/// Serves MCP over standard input/output until the transport closes.
pub async fn run_stdio() -> Result<(), Box<dyn std::error::Error>> {
    let server = ArsenaleroServer::from_environment()
        .map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidInput, error.code()))?;
    let service = server.serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use rmcp::model::CallToolRequestParams;

    use super::{ServerState, dispatch, tool_definitions};

    #[test]
    fn lists_exactly_the_five_sdd_tools_with_schemas() {
        let tools = tool_definitions();
        assert_eq!(
            tools
                .iter()
                .map(|tool| tool.name.as_ref())
                .collect::<Vec<_>>(),
            [
                "arsenal_init",
                "arsenal_stage",
                "arsenal_issue",
                "arsenal_attest",
                "arsenal_reconcile"
            ]
        );
        assert!(tools.iter().all(|tool| tool.output_schema.is_some()));
    }

    #[test]
    fn schema_snapshots_cover_every_input_and_output() {
        let snapshots: &[(&str, &str, &str)] = &[
            (
                "arsenal_init",
                r##"{"$schema":"https://json-schema.org/draft/2020-12/schema","additionalProperties":false,"properties":{"operation":{"type":"string"},"skill_root":{"type":"string"},"task_summary":{"type":"string"}},"required":["skill_root","task_summary","operation"],"type":"object"}"##,
                r##"{"$defs":{"EvidenceContractOutput":{"properties":{"minimum":{"type":"string"},"supported_levels":{"items":{"type":"string"},"type":"array"}},"required":["minimum","supported_levels"],"type":"object"},"ResourceOutput":{"properties":{"evidence_contract":{"$ref":"#/$defs/EvidenceContractOutput"},"obligation":{"type":"string"},"path":{"type":"string"},"purpose":{"type":"string"},"resource_id":{"type":"string"}},"required":["resource_id","path","purpose","obligation","evidence_contract"],"type":"object"},"SkillOutput":{"properties":{"digest":{"type":"string"},"name":{"type":"string"}},"required":["name","digest"],"type":"object"}},"$schema":"https://json-schema.org/draft/2020-12/schema","properties":{"case_id":{"type":"string"},"orphan_files":{"items":{"type":"string"},"type":"array"},"required_resource_ids":{"items":{"type":"string"},"type":"array"},"resources":{"items":{"$ref":"#/$defs/ResourceOutput"},"type":"array"},"skill":{"$ref":"#/$defs/SkillOutput"},"status":{"type":"string"},"unresolved":{"items":{"type":"string"},"type":"array"},"warnings":{"items":{"type":"string"},"type":"array"}},"required":["case_id","skill","required_resource_ids","resources","unresolved","orphan_files","warnings","status"],"type":"object"}"##,
            ),
            (
                "arsenal_stage",
                r##"{"$schema":"https://json-schema.org/draft/2020-12/schema","additionalProperties":false,"properties":{"case_id":{"type":"string"},"current_intent":{"type":"string"},"stage":{"type":"string"}},"required":["case_id","stage","current_intent"],"type":"object"}"##,
                r##"{"$schema":"https://json-schema.org/draft/2020-12/schema","properties":{"already_issued":{"items":{"type":"string"},"type":"array"},"recommended_now":{"items":{"type":"string"},"type":"array"},"required_now":{"items":{"type":"string"},"type":"array"},"unresolved_relevant":{"items":{"type":"string"},"type":"array"}},"required":["required_now","recommended_now","already_issued","unresolved_relevant"],"type":"object"}"##,
            ),
            (
                "arsenal_issue",
                r##"{"$schema":"https://json-schema.org/draft/2020-12/schema","additionalProperties":false,"properties":{"case_id":{"type":"string"},"resource_ids":{"items":{"type":"string"},"maxItems":4,"minItems":1,"type":"array"}},"required":["case_id","resource_ids"],"type":"object"}"##,
                r##"{"$defs":{"EvidenceContractOutput":{"properties":{"minimum":{"type":"string"},"supported_levels":{"items":{"type":"string"},"type":"array"}},"required":["minimum","supported_levels"],"type":"object"},"IssuedResourceOutput":{"properties":{"content":{"type":"string"},"digest":{"type":"string"},"evidence_contract":{"$ref":"#/$defs/EvidenceContractOutput"},"purpose":{"type":"string"},"receipt_id":{"type":"string"},"resource_id":{"type":"string"}},"required":["receipt_id","resource_id","digest","purpose","content","evidence_contract"],"type":"object"}},"$schema":"https://json-schema.org/draft/2020-12/schema","properties":{"resources":{"items":{"$ref":"#/$defs/IssuedResourceOutput"},"type":"array"}},"required":["resources"],"type":"object"}"##,
            ),
            (
                "arsenal_attest",
                r##"{"$defs":{"AttestationInput":{"additionalProperties":false,"properties":{"evidence":{"items":{"$ref":"#/$defs/EvidenceInput"},"type":"array"},"receipt_id":{"type":"string"},"usage":{"minLength":1,"type":"string"}},"required":["receipt_id","usage"],"type":"object"},"EvidenceInput":{"additionalProperties":false,"properties":{"reference":{"minLength":1,"type":"string"},"type":{"type":"string"}},"required":["type","reference"],"type":"object"}},"$schema":"https://json-schema.org/draft/2020-12/schema","additionalProperties":false,"properties":{"attestations":{"items":{"$ref":"#/$defs/AttestationInput"},"maxItems":16,"type":"array"},"case_id":{"type":"string"}},"required":["case_id","attestations"],"type":"object"}"##,
                r##"{"$defs":{"AttestationOutput":{"properties":{"attained_evidence_level":{"type":"string"},"receipt_id":{"type":"string"}},"required":["receipt_id","attained_evidence_level"],"type":"object"}},"$schema":"https://json-schema.org/draft/2020-12/schema","properties":{"attestations":{"items":{"$ref":"#/$defs/AttestationOutput"},"type":"array"}},"required":["attestations"],"type":"object"}"##,
            ),
            (
                "arsenal_reconcile",
                r##"{"$schema":"https://json-schema.org/draft/2020-12/schema","additionalProperties":false,"properties":{"case_id":{"type":"string"}},"required":["case_id"],"type":"object"}"##,
                r##"{"$defs":{"AttestationBreakdownOutput":{"properties":{"artifact_referenced":{"format":"uint","minimum":0,"type":"integer"},"externally_verified":{"format":"uint","minimum":0,"type":"integer"},"self_report_only":{"format":"uint","minimum":0,"type":"integer"}},"required":["self_report_only","artifact_referenced","externally_verified"],"type":"object"},"EvidenceCoverageOutput":{"properties":{"artifact_referenced":{"format":"uint","minimum":0,"type":"integer"},"expected_artifact_references":{"format":"uint","minimum":0,"type":"integer"},"ratio":{"format":"double","type":"number"}},"required":["expected_artifact_references","artifact_referenced","ratio"],"type":"object"},"PerResourceEvidenceOutput":{"properties":{"attained_evidence_level":{"type":["string","null"]},"resource_id":{"type":"string"},"verification_status":{"type":"string"}},"required":["resource_id","verification_status"],"type":"object"},"ProtocolCompletionOutput":{"properties":{"attested":{"format":"uint","minimum":0,"type":"integer"},"issued":{"format":"uint","minimum":0,"type":"integer"},"ratio":{"format":"double","type":"number"},"required":{"format":"uint","minimum":0,"type":"integer"}},"required":["required","issued","attested","ratio"],"type":"object"},"VerificationOutput":{"properties":{"status":{"type":"string"},"verified_resources":{"format":"uint","minimum":0,"type":"integer"}},"required":["status","verified_resources"],"type":"object"}},"$schema":"https://json-schema.org/draft/2020-12/schema","properties":{"attestation_breakdown":{"$ref":"#/$defs/AttestationBreakdownOutput"},"disclaimer":{"type":"string"},"evidence_coverage":{"$ref":"#/$defs/EvidenceCoverageOutput"},"missing_attestations":{"items":{"type":"string"},"type":"array"},"per_resource_evidence":{"items":{"$ref":"#/$defs/PerResourceEvidenceOutput"},"type":"array"},"protocol_completion":{"$ref":"#/$defs/ProtocolCompletionOutput"},"required_but_never_issued":{"items":{"type":"string"},"type":"array"},"resource_modifications_post_attestation":{"items":{"type":"string"},"type":"array"},"stale_receipts":{"items":{"type":"string"},"type":"array"},"status":{"type":"string"},"unresolved_resources":{"items":{"type":"string"},"type":"array"},"verification":{"$ref":"#/$defs/VerificationOutput"}},"required":["status","protocol_completion","evidence_coverage","attestation_breakdown","verification","missing_attestations","required_but_never_issued","stale_receipts","resource_modifications_post_attestation","unresolved_resources","per_resource_evidence","disclaimer"],"type":"object"}"##,
            ),
        ];

        for (name, input, output) in snapshots {
            let definition = tool_definitions()
                .into_iter()
                .find(|definition| definition.name == *name)
                .expect("tool exists");
            let actual_input =
                serde_json::to_value(&definition.input_schema).expect("input schema serializes");
            let actual_output =
                serde_json::to_value(definition.output_schema.as_ref().expect("output schema"))
                    .expect("output schema serializes");
            assert_eq!(
                actual_input,
                serde_json::from_str::<serde_json::Value>(input).expect("input snapshot JSON"),
                "complete input schema snapshot for {name}"
            );
            assert_eq!(
                actual_output,
                serde_json::from_str::<serde_json::Value>(output).expect("output snapshot JSON"),
                "complete output schema snapshot for {name}"
            );
        }
    }

    #[test]
    fn maps_domain_errors_to_error_results() {
        let state = ServerState::empty();
        let arguments = serde_json::json!({
            "case_id": "00000000-0000-7000-8000-000000000000",
            "stage": "verification",
            "current_intent": "run checks"
        })
        .as_object()
        .cloned()
        .expect("object arguments");

        let result = dispatch(
            &state,
            CallToolRequestParams::new("arsenal_stage").with_arguments(arguments),
        );

        assert_eq!(result.is_error, Some(true));
        assert_eq!(
            result
                .structured_content
                .as_ref()
                .and_then(|value| value.get("error_code"))
                .and_then(serde_json::Value::as_str),
            Some("CASE_UNKNOWN")
        );
    }

    #[test]
    fn input_schemas_are_strict_and_enforce_batch_and_text_limits() {
        let tools = tool_definitions();
        for tool in &tools {
            assert_eq!(
                tool.input_schema.get("additionalProperties"),
                Some(&serde_json::json!(false)),
                "input schema for {} must reject unknown fields",
                tool.name
            );
        }

        let issue = tools
            .iter()
            .find(|tool| tool.name == "arsenal_issue")
            .expect("issue tool");
        assert_eq!(
            issue.input_schema["properties"]["resource_ids"]["maxItems"],
            serde_json::json!(4)
        );
        assert_eq!(
            issue.input_schema["properties"]["resource_ids"]["minItems"],
            serde_json::json!(1)
        );

        let attest = tools
            .iter()
            .find(|tool| tool.name == "arsenal_attest")
            .expect("attest tool");
        assert_eq!(
            attest.input_schema["properties"]["attestations"]["maxItems"],
            serde_json::json!(16)
        );
        assert_eq!(
            attest.input_schema["$defs"]["AttestationInput"]["properties"]["usage"]["minLength"],
            serde_json::json!(1)
        );
        assert_eq!(
            attest.input_schema["$defs"]["EvidenceInput"]["properties"]["reference"]["minLength"],
            serde_json::json!(1)
        );
    }

    #[test]
    fn rejects_unknown_tool_arguments_as_invalid_arguments() {
        let state = ServerState::empty();
        let arguments = serde_json::json!({
            "skill_root": "/tmp/skill",
            "task_summary": "task",
            "operation": "implementation",
            "unexpected": true
        })
        .as_object()
        .cloned()
        .expect("object arguments");

        let result = dispatch(
            &state,
            CallToolRequestParams::new("arsenal_init").with_arguments(arguments),
        );

        assert_eq!(result.is_error, Some(true));
        assert_eq!(
            result
                .structured_content
                .as_ref()
                .and_then(|value| value.get("error_code"))
                .and_then(serde_json::Value::as_str),
            Some("INVALID_ARGUMENTS")
        );
        let wire = serde_json::to_value(&result).expect("result serializes");
        assert_eq!(wire.get("isError"), Some(&serde_json::json!(true)));
        let text = wire["content"][0]["text"]
            .as_str()
            .expect("serialized JSON text content");
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(text).expect("text is JSON"),
            result
                .structured_content
                .clone()
                .expect("structured content")
        );
    }
}

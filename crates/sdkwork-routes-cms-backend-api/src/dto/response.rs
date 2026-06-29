use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApiResponse<T: Serialize> {
    pub ok: bool,
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ProblemDetail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProblemDetail {
    #[serde(rename = "type")]
    pub problem_type: String,
    pub title: String,
    pub status: u16,
    pub detail: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "traceId")]
    pub trace_id: Option<String>,
}
    pub items: Vec<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommandResponse {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SiteResponse {
    pub id: String,
    pub uuid: String,
    pub tenant_id: String,
    pub organization_id: String,
    pub code: String,
    pub name: String,
    pub default_locale: String,
    pub settings_json: String,
    pub status: i32,
    pub version: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChannelResponse {
    pub id: String,
    pub site_id: String,
    pub code: String,
    pub name: String,
    pub channel_kind: String,
    pub status: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContentTypeResponse {
    pub id: String,
    pub site_id: String,
    pub code: String,
    pub name: String,
    pub content_kind: String,
    pub schema_version: String,
    pub status: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContentFieldResponse {
    pub id: String,
    pub content_type_id: String,
    pub code: String,
    pub name: String,
    pub field_kind: String,
    pub required: bool,
    pub searchable: bool,
    pub filterable: bool,
    pub sortable: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TaxonomyResponse {
    pub id: String,
    pub site_id: String,
    pub code: String,
    pub name: String,
    pub taxonomy_kind: String,
    pub status: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TaxonomyTermResponse {
    pub id: String,
    pub taxonomy_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    pub code: String,
    pub slug: String,
    pub name: String,
    pub path: String,
    pub status: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EntryResponse {
    pub id: String,
    pub uuid: String,
    pub site_id: String,
    pub content_type_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    pub locale: String,
    pub title: String,
    pub slug: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    pub entry_status: String,
    pub publication_status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_at: Option<String>,
    pub version: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EntryVersionResponse {
    pub id: String,
    pub entry_id: String,
    pub version_no: String,
    pub version_kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaRefResponse {
    pub id: String,
    pub role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drive_space_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drive_node_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drive_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_resource_id: Option<String>,
    pub media_snapshot_json: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PublishSnapshotResponse {
    pub id: String,
    pub owner_type: String,
    pub owner_id: String,
    pub snapshot_payload_json: String,
    pub status: i32,
    pub published_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PageResponse {
    pub id: String,
    pub site_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    pub locale: String,
    pub path: String,
    pub title: String,
    pub publication_status: String,
    pub version: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FeedResponse {
    pub id: String,
    pub site_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    pub code: String,
    pub name: String,
    pub feed_kind: String,
    pub locale: String,
    pub status: i32,
    pub version: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FeedRuleResponse {
    pub id: String,
    pub feed_id: String,
    pub rule_kind: String,
    pub condition_json: String,
    pub sort_json: String,
    pub enabled: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FeedItemResponse {
    pub id: String,
    pub feed_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_url: Option<String>,
    pub item_kind: String,
    pub pinned: bool,
    pub sort_order: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FeedSnapshotResponse {
    pub id: String,
    pub feed_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_snapshot_id: Option<String>,
    pub snapshot_version: String,
    pub item_count: i32,
    pub items_json: String,
    pub status: i32,
    pub published_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuditLogResponse {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_id: Option<String>,
    pub actor_user_id: String,
    pub action: String,
    pub resource_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    pub before_json: String,
    pub after_json: String,
    pub created_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OutboxEventResponse {
    pub id: String,
    pub aggregate_type: String,
    pub aggregate_id: String,
    pub event_type: String,
    pub payload_json: String,
    pub status: i32,
    pub attempt_count: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_attempt_at: Option<String>,
    pub created_at: String,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T, request_id: Option<String>) -> Self {
        Self {
            ok: true,
            data: Some(data),
            error: None,
            request_id,
        }
    }

    pub fn error(problem: ProblemDetail, request_id: Option<String>) -> Self {
        Self {
            ok: false,
            data: None,
            error: Some(problem),
            request_id,
        }
    }
}

impl ProblemDetail {
    pub fn not_found(resource: &str, trace_id: Option<String>) -> Self {
        Self {
            problem_type: "https://sdkwork.com/errors/not-found".to_string(),
            title: "Not Found".to_string(),
            status: 404,
            detail: format!("{} not found", resource),
            instance: None,
            trace_id,
        }
    }

    pub fn permission_denied(permission: &str, trace_id: Option<String>) -> Self {
        Self {
            problem_type: "https://sdkwork.com/errors/permission-denied".to_string(),
            title: "Permission Denied".to_string(),
            status: 403,
            detail: format!("Missing required permission: {}", permission),
            instance: None,
            trace_id,
        }
    }

    pub fn validation(message: &str, trace_id: Option<String>) -> Self {
        Self {
            problem_type: "https://sdkwork.com/errors/validation".to_string(),
            title: "Validation Error".to_string(),
            status: 400,
            detail: message.to_string(),
            instance: None,
            trace_id,
        }
    }

    pub fn conflict(message: &str, trace_id: Option<String>) -> Self {
        Self {
            problem_type: "https://sdkwork.com/errors/conflict".to_string(),
            title: "Conflict".to_string(),
            status: 409,
            detail: message.to_string(),
            instance: None,
            trace_id,
        }
    }

    pub fn optimistic_lock_conflict(resource: &str, resource_id: i64, expected_version: i64, trace_id: Option<String>) -> Self {
        Self {
            problem_type: "https://sdkwork.com/errors/optimistic-lock-conflict".to_string(),
            title: "Optimistic Lock Conflict".to_string(),
            status: 409,
            detail: format!("{} id={} version conflict, expected version {}", resource, resource_id, expected_version),
            instance: None,
            trace_id,
        }
    }

    pub fn internal(message: &str, trace_id: Option<String>) -> Self {
        Self {
            problem_type: "https://sdkwork.com/errors/internal".to_string(),
            title: "Internal Server Error".to_string(),
            status: 500,
            detail: message.to_string(),
            instance: None,
            trace_id,
        }
    }
}

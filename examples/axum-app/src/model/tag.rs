use serde::{Deserialize, Serialize};
use zino::prelude::*;
use zino_derive::{Model, ModelAccessor, ModelHooks, Schema};

/// The `tag` model.
#[derive(Debug, Clone, Default, Serialize, Deserialize, Schema, ModelAccessor, ModelHooks, Model)]
#[serde(rename_all = "snake_case")]
#[serde(default)]
pub struct Tag {
    // Basic fields.
    #[schema(primary_key, auto_increment, readonly, comment = "Tag ID")]
    id: i64,
    #[schema(not_null, index_type = "text", comment = "Tag name")]
    name: String,
    #[schema(default_value = "Active", index_type = "hash")]
    status: String,
    #[schema(index_type = "text")]
    description: String,

    // Info fields.
    #[schema(not_null, comment = "Tag category")]
    category: String,
    #[schema(snapshot, reference = "Tag", comment = "Optional parent tag")]
    parent_id: Option<i64>,

    // Extensions.
    content: Map,
    extra: Map,

    // Revisions.
    #[schema(readonly, default_value = "now", index_type = "btree")]
    created_at: DateTime,
    #[schema(default_value = "now", index_type = "btree")]
    updated_at: DateTime,
    version: u64,
}

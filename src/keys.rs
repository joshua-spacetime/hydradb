pub use hydradb_graph_slate::keys::*;

/// Prefix the client layer puts on idempotency keys it mints itself for
/// requests that carry no caller key. No client can retry with such a key.
pub(crate) const SERVER_GENERATED_IDEMPOTENCY_KEY_PREFIX: &str = "server-v1-";

/// Whether an [`idempotency`] record key holds a server-generated key.
pub(crate) fn is_server_generated_idempotency(key: &str) -> bool {
    key.splitn(5, '/')
        .nth(4)
        .is_some_and(|key| key.starts_with(SERVER_GENERATED_IDEMPOTENCY_KEY_PREFIX))
}

/// Immutable collection layout, written by the offline BYOG importer.
pub(crate) const CYPHER_ID_SEMANTICS: &str = "meta/cypher_id_semantics";
pub(crate) const CYPHER_ID_PROPERTIES_V1: &[u8] = b"properties-v1";

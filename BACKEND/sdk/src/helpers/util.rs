use uuid::Uuid;

pub fn sqlite_test_document(id: Uuid) -> String {
    return format!("sqlite://test/sqlite/test-{id}.sqlite?mode=rwc");
}
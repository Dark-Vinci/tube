use uuid::Uuid;

pub fn sqlite_test_document(id: Uuid) -> String {
    return format!("sqlite://tests/sqlite/tests-{id}.sqlite?mode=rwc");
}
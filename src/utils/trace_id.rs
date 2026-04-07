use uuid::Uuid;

pub fn generate_trace_id() -> String {
    Uuid::new_v4().simple().to_string()
}

pub fn generate_request_id() -> String {
    format!("req-{}", Uuid::new_v4().simple())
}

pub fn generate_connection_id() -> String {
    Uuid::new_v4().hyphenated().to_string()
}

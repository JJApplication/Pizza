use crate::constants::{
    HEADER_DIRECT_ACCESS, HEADER_INTERNAL_FLAG, HEADER_INTERNAL_HOST, HEADER_TRACE_ID,
};
use http::HeaderMap;

pub fn get_trace_id(headers: &HeaderMap) -> Option<String> {
    headers
        .get(HEADER_TRACE_ID)
        .and_then(|v| v.to_str().ok())
        .map(String::from)
}

pub fn set_trace_id(headers: &mut HeaderMap, trace_id: &str) {
    if let Ok(val) = trace_id.parse() {
        headers.insert(HEADER_TRACE_ID, val);
    }
}

pub fn get_internal_flag(headers: &HeaderMap) -> Option<String> {
    headers
        .get(HEADER_INTERNAL_FLAG)
        .and_then(|v| v.to_str().ok())
        .map(String::from)
}

pub fn set_internal_flag(headers: &mut HeaderMap, flag: &str) {
    if let Ok(val) = flag.parse() {
        headers.insert(HEADER_INTERNAL_FLAG, val);
    }
}

pub fn get_internal_host(headers: &HeaderMap) -> Option<String> {
    headers
        .get(HEADER_INTERNAL_HOST)
        .and_then(|v| v.to_str().ok())
        .map(String::from)
}

pub fn set_internal_host(headers: &mut HeaderMap, host: &str) {
    if let Ok(val) = host.parse() {
        headers.insert(HEADER_INTERNAL_HOST, val);
    }
}

pub fn get_direct_access_host(headers: &HeaderMap) -> Option<String> {
    headers
        .get(HEADER_DIRECT_ACCESS)
        .and_then(|v| v.to_str().ok())
        .map(String::from)
}

pub fn is_grpc_request(headers: &HeaderMap) -> bool {
    headers
        .get(http::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .map(|ct| ct.starts_with("application/grpc"))
        .unwrap_or(false)
}

pub fn is_grpc_web_request(headers: &HeaderMap) -> bool {
    headers
        .get(http::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .map(|ct| ct.starts_with("application/grpc-web"))
        .unwrap_or(false)
}

pub fn copy_hop_by_hop_headers(src: &HeaderMap, dst: &mut HeaderMap) {
    let hop_headers = [
        "connection",
        "keep-alive",
        "proxy-authenticate",
        "proxy-authorization",
        "te",
        "trailers",
        "transfer-encoding",
        "upgrade",
    ];
    for h in &hop_headers {
        if let Some(val) = src.get(*h) {
            dst.insert(*h, val.clone());
        }
    }
}

pub fn remove_hop_by_hop_headers(headers: &mut HeaderMap) {
    let hop_headers = [
        "connection",
        "keep-alive",
        "proxy-authenticate",
        "proxy-authorization",
        "te",
        "trailers",
        "transfer-encoding",
        "upgrade",
    ];
    for h in &hop_headers {
        headers.remove(*h);
    }
}

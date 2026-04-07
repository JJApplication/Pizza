use axum::Router;
use axum::routing::get;
use axum::http::{Request, StatusCode};
use axum::response::Response;
use axum::body::Body;
use tower_http::services::ServeDir;
use std::path::PathBuf;
use std::sync::Arc;
use crate::frontend::file_cache::FileCache;

pub struct FrontendServer {
    root: PathBuf,
    file_cache: Arc<FileCache>,
    try_files_fallback: Option<String>,
    directory_listing: bool,
    custom_404: Option<PathBuf>,
    custom_500: Option<PathBuf>,
}

impl FrontendServer {
    pub fn new(root: PathBuf, cache_enabled: bool, cache_ttl_secs: u64) -> Self {
        Self {
            root,
            file_cache: Arc::new(FileCache::new(cache_enabled, cache_ttl_secs)),
            try_files_fallback: Some("/index.html".to_string()),
            directory_listing: false,
            custom_404: None,
            custom_500: None,
        }
    }

    pub fn with_try_files_fallback(mut self, fallback: String) -> Self {
        self.try_files_fallback = Some(fallback);
        self
    }

    pub fn with_directory_listing(mut self, enabled: bool) -> Self {
        self.directory_listing = enabled;
        self
    }

    pub fn with_custom_404(mut self, path: PathBuf) -> Self {
        self.custom_404 = Some(path);
        self
    }

    pub fn with_custom_500(mut self, path: PathBuf) -> Self {
        self.custom_500 = Some(path);
        self
    }

    pub fn into_router(self) -> Router {
        let static_service = ServeDir::new(&self.root)
            .append_index_html_on_directories(self.directory_listing);

        let fallback = self.try_files_fallback.clone();
        let root = self.root.clone();

        Router::new()
            .nest_service("/", static_service)
            .fallback(move |req| spa_fallback_handler(req, fallback.clone(), root.clone()))
    }

    pub fn clear_cache(&self) {
        self.file_cache.clear();
    }

    pub fn cache_stats(&self) -> (u64, u64) {
        self.file_cache.stats()
    }
}

async fn spa_fallback_handler(
    req: Request<Body>,
    fallback: Option<String>,
    root: PathBuf,
) -> Response<Body> {
    if let Some(fb) = fallback {
        let index_path = root.join(fb.trim_start_matches('/'));
        match tokio::fs::read(&index_path).await {
            Ok(content) => {
                let mime = mime_guess::from_path(&index_path).first_or_octet_stream();
                Response::builder()
                    .header(http::header::CONTENT_TYPE, mime.as_ref())
                    .body(Body::from(content))
                    .unwrap()
            }
            Err(_) => {
                Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::from("404 Not Found"))
                    .unwrap()
            }
        }
    } else {
        let _ = req;
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("404 Not Found"))
            .unwrap()
    }
}

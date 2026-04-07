use parking_lot::RwLock;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ErrorPage {
    pub status_code: u16,
    pub content: String,
    pub content_type: String,
}

pub struct ErrorPageManager {
    pages: RwLock<HashMap<u16, ErrorPage>>,
}

impl ErrorPageManager {
    pub fn new() -> Self {
        let mut manager = Self {
            pages: RwLock::new(HashMap::new()),
        };
        manager.load_defaults();
        manager
    }

    fn load_defaults(&mut self) {
        self.pages.write().insert(
            400,
            ErrorPage {
                status_code: 400,
                content: "<html><body><h1>400 Bad Request</h1></body></html>".to_string(),
                content_type: "text/html".to_string(),
            },
        );
        self.pages.write().insert(
            403,
            ErrorPage {
                status_code: 403,
                content: "<html><body><h1>403 Forbidden</h1></body></html>".to_string(),
                content_type: "text/html".to_string(),
            },
        );
        self.pages.write().insert(
            404,
            ErrorPage {
                status_code: 404,
                content: "<html><body><h1>404 Not Found</h1></body></html>".to_string(),
                content_type: "text/html".to_string(),
            },
        );
        self.pages.write().insert(
            500,
            ErrorPage {
                status_code: 500,
                content: "<html><body><h1>500 Internal Server Error</h1></body></html>".to_string(),
                content_type: "text/html".to_string(),
            },
        );
        self.pages.write().insert(
            502,
            ErrorPage {
                status_code: 502,
                content: "<html><body><h1>502 Bad Gateway</h1></body></html>".to_string(),
                content_type: "text/html".to_string(),
            },
        );
        self.pages.write().insert(
            503,
            ErrorPage {
                status_code: 503,
                content: "<html><body><h1>503 Service Unavailable</h1></body></html>".to_string(),
                content_type: "text/html".to_string(),
            },
        );
        self.pages.write().insert(
            504,
            ErrorPage {
                status_code: 504,
                content: "<html><body><h1>504 Gateway Timeout</h1></body></html>".to_string(),
                content_type: "text/html".to_string(),
            },
        );
    }

    pub fn register(&self, status_code: u16, content: String, content_type: String) {
        self.pages.write().insert(
            status_code,
            ErrorPage {
                status_code,
                content,
                content_type,
            },
        );
    }

    pub fn get_page(&self, status_code: u16) -> Option<ErrorPage> {
        self.pages.read().get(&status_code).cloned()
    }

    pub fn minify_all(&self) {
        let mut pages = self.pages.write();
        for page in pages.values_mut() {
            page.content = minify_html(&page.content);
        }
    }
}

impl Default for ErrorPageManager {
    fn default() -> Self {
        Self::new()
    }
}

fn minify_html(html: &str) -> String {
    html.split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
        .replace("> <", "><")
}

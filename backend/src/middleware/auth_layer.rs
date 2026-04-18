use axum::{extract::Request, middleware::Next, response::Response, http::{StatusCode, header}};

pub async fn check_auth(req: Request, next: Next) -> Result<Response, StatusCode> {
    let auth_header = req.headers().get(header::AUTHORIZATION).and_then(|h| h.to_str().ok());
    let master_password = std::env::var("MASTER_PASSWORD").unwrap_or_else(|_| "miransas_default".to_string());

    if let Some(auth_value) = auth_header {
        if auth_value == master_password {
            return Ok(next.run(req).await);
        }
    }
    Err(StatusCode::UNAUTHORIZED)
}
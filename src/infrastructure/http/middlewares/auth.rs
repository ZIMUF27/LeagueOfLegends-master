use axum::{
    extract::Request,
    http::{header, StatusCode},
    response::Response,
    middleware::Next,
};

pub async fn auth(req: Request,next: Next) -> Result<Response, StatusCode> {
 let header : &str = req.headers().get(header::AUTHORIZATION).and_then(|value| value.to_str().ok()).ok_or(StatusCode::UNAUTHORIZED);
 let token = header.strip_prefix("Bearer ").ok_or(StatusCode::UNAUTHORIZED)?;
 let secret: String = get_user_secret().map_err(|_| StatusCode::UNAUTHORIZED)?;
 let claims: Claims = verify_token(&secret, token).map_err(|_| StatusCode::UNAUTHORIZED)?;
 let user_id: i32 = claims.sub.parse().map_err(|_| StatusCode::UNAUTHORIZED)?;
 req.extensions_mut().insert(user_id);
 Ok(next.run(req).await)
}
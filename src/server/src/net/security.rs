use actix_web::middleware::DefaultHeaders;

pub fn security_headers() -> DefaultHeaders {
    DefaultHeaders::new()
        .add((
            "Content-Security-Policy",
            "default-src 'self'; style-src 'self' 'unsafe-inline'; script-src 'self' 'unsafe-eval'",
        ))
        .add(("X-Content-Type-Options", "nosniff"))
        .add(("X-Frame-Options", "DENY"))
}

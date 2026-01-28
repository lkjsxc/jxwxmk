#[cfg(test)]
mod tests {
    use crate::routes;
    use actix_web::{test, App, web};

    #[actix_web::test]
    async fn test_health_check() {
        let app = test::init_service(App::new().route("/health", web::get().to(routes::health))).await;
        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let body = test::read_body(resp).await;
        assert_eq!(body, "OK");
    }
}
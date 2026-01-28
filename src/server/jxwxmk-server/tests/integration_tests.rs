use actix_web::{test, web, App};
use std::sync::Arc;

#[actix_rt::test]
async fn test_health_endpoint() {
    let app = test::init_service(
        App::new().service(jxwxmk_server::handlers::health)
    ).await;

    let req = test::TestRequest::get()
        .uri("/health")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    assert_eq!(body, "OK");
}

#[actix_rt::test]
async fn test_metrics_endpoint() {
    let app = test::init_service(
        App::new().service(jxwxmk_server::handlers::metrics)
    ).await;

    let req = test::TestRequest::get()
        .uri("/metrics")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("jxwxmk_"));
}

#[actix_rt::test]
async fn test_session_claim_endpoint() {
    let app = test::init_service(
        App::new().service(jxwxmk_server::handlers::session_claim)
    ).await;

    let req = test::TestRequest::post()
        .uri("/session/claim")
        .set_json(serde_json::json!({
            "player_id": "550e8400-e29b-41d4-a716-446655440000"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body.get("id").is_some());
    assert!(body.get("token").is_some());
}

#[actix_rt::test]
async fn test_index_endpoint() {
    let app = test::init_service(
        App::new().service(jxwxmk_server::handlers::serve_index)
    ).await;

    let req = test::TestRequest::get()
        .uri("/")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("JXWXMK"));
}

#[actix_rt::test]
async fn test_static_asset_css() {
    let app = test::init_service(
        App::new().service(jxwxmk_server::handlers::serve_asset)
    ).await;

    let req = test::TestRequest::get()
        .uri("/styles.css")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let content_type = resp.headers().get("content-type").unwrap();
    assert_eq!(content_type, "text/css");
}

#[actix_rt::test]
async fn test_static_asset_not_found() {
    let app = test::init_service(
        App::new().service(jxwxmk_server::handlers::serve_asset)
    ).await;

    let req = test::TestRequest::get()
        .uri("/nonexistent.file")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}

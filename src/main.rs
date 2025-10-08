 use axum::{
 	routing::post,
 	Json, Router,
 };
 use axum::http::{HeaderMap, HeaderValue};
 use axum::response::IntoResponse;
 use axum::response::Response;
 use serde::{Deserialize, Serialize};
 use std::net::SocketAddr;
 use tokio::time::{sleep, Duration};
 use async_stream::stream;

 #[derive(Debug, Deserialize, Serialize)]
 struct UIMessagePart {
 	// In Python: Dict[str, Any]. We just keep raw JSON here.
 	#[serde(flatten)]
 	other: serde_json::Value,
 }

 #[derive(Debug, Deserialize, Serialize)]
 struct UIMessage {
 	role: String,
 	parts: Vec<serde_json::Map<String, serde_json::Value>>, // approximate structure
 }

 #[derive(Debug, Deserialize, Serialize)]
 struct ChatRequest {
 	chat_id: String,
 	model: String,
 	messages: Vec<UIMessage>,
 }

 #[tokio::main]
 async fn main() {
 	tracing_subscriber::fmt()
 		.with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
 		.init();

 	let app = Router::new().route("/api/agent/chat", post(chat_with_agent));

 	let addr: SocketAddr = "127.0.0.1:5000".parse().unwrap();
 	tracing::info!("listening on {}", addr);
	let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
 	axum::serve(listener, app)
 	.await
 	.unwrap();
 }

 async fn chat_with_agent(Json(_request): Json<ChatRequest>) -> Response {
 	// Static response string mirroring Python implementation
 	let static_response = "这是一个来自Rust Axum Agent的静态回复。我可以处理您的请求并提供帮助。";
 	let chunk_size = 5usize;

 	let s = stream! {
 		// First chunk: text-start
 		yield Ok::<_, std::io::Error>(format!("{{\"type\":\"text-start\",\"id\":\"agent-response\"}}\n"));

 		let mut index = 0usize;
 		while index < static_response.len() {
 			let end = (index + chunk_size).min(static_response.len());
 			let chunk = &static_response[index..end];
 			let ndjson_line = serde_json::json!({
 				"type": "text-delta",
 				"id": "agent-response",
 				"delta": chunk,
 			})
 			.to_string() + "\n";
 			yield Ok::<_, std::io::Error>(ndjson_line);
 			index = end;
 			sleep(Duration::from_millis(100)).await;
 		}

 		// Final: text-end
 		yield Ok::<_, std::io::Error>(format!("{{\"type\":\"text-end\",\"id\":\"agent-response\"}}\n"));
 	};

 	let body = axum::body::Body::from_stream(s);
 	Response::builder()
 		.status(200)
 		.header(axum::http::header::CONTENT_TYPE, HeaderValue::from_static("application/x-ndjson"))
 		.header(axum::http::header::CACHE_CONTROL, HeaderValue::from_static("no-cache"))
 		.header(axum::http::header::CONNECTION, HeaderValue::from_static("keep-alive"))
 		.body(body)
 		.unwrap()
 }

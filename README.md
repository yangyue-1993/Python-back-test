# Rust-back-test
基于 Rust + Axum 重写的后端，功能等价于原 Python FastAPI 示例：

- **接口**: `POST /api/agent/chat`
- **行为**: 返回符合 toUIMessageStream() 的 NDJSON 流式数据：
  - 首个事件：`{"type":"text-start","id":"agent-response"}`
  - 中间多次事件：`{"type":"text-delta","id":"agent-response","delta":"..."}`
  - 结束事件：`{"type":"text-end","id":"agent-response"}`

## 运行

```bash
cargo run
```

服务默认监听 `127.0.0.1:5000`。

## 依赖（需通过 Cargo 自动下载）

本项目使用以下第三方 crate（运行 `cargo build`/`cargo run` 会自动下载）：

- axum = "0.7"
- tokio = { version = "1", features = ["full"] }
- serde = { version = "1", features = ["derive"] }
- serde_json = "1"
- tracing = "0.1"
- tracing-subscriber = { version = "0.3", features = ["env-filter"] }
- async-stream = "0.3"

无需手动安装 Python 依赖；Rust 编译器和 Cargo 会自动处理 crate 下载与构建。

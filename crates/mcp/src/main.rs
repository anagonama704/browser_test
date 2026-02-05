//! integrity_mcp MCP Server
//!
//! stdio経由でMCPプロトコルを実装するサーバ

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{self, BufRead, Write};

/// MCPサーバのバージョン
const SERVER_VERSION: &str = "0.1.0";

/// プロトコルバージョン
const PROTOCOL_VERSION: &str = "2024-11-05";

/// 最大入力サイズ（バイト）
const MAX_INPUT_SIZE: usize = 1_048_576;

/// JSONRPCリクエスト
#[derive(Debug, Deserialize)]
struct JsonRpcRequest {
    #[allow(dead_code)]
    jsonrpc: String,
    id: Option<serde_json::Value>,
    method: String,
    #[serde(default)]
    params: serde_json::Value,
}

/// JSONRPCレスポンス
#[derive(Debug, Serialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    id: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<JsonRpcError>,
}

/// JSONRPCエラー
#[derive(Debug, Serialize)]
struct JsonRpcError {
    code: i32,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<serde_json::Value>,
}

impl JsonRpcResponse {
    fn success(id: Option<serde_json::Value>, result: serde_json::Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(result),
            error: None,
        }
    }

    fn error(id: Option<serde_json::Value>, code: i32, message: String) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: None,
            error: Some(JsonRpcError {
                code,
                message,
                data: None,
            }),
        }
    }
}

/// MCPサーバ
struct McpServer {
    /// リソースのマッピング
    resources: HashMap<String, String>,
}

impl McpServer {
    fn new() -> Result<Self> {
        let mut resources = HashMap::new();

        // リソースファイルを読み込み
        let base_path =
            std::env::var("INTEGRITY_RULES_PATH").unwrap_or_else(|_| ".cursor/rules".to_string());

        // project_v0_1.md
        let project_content = std::fs::read_to_string(format!("{}/project_v0_1.md", base_path))
            .unwrap_or_else(|_| include_str!("../../../.cursor/rules/project_v0_1.md").to_string());
        resources.insert("integrity://project".to_string(), project_content);

        // philosophy_v0_1.md
        let philosophy_content =
            std::fs::read_to_string(format!("{}/philosophy_v0_1.md", base_path)).unwrap_or_else(
                |_| include_str!("../../../.cursor/rules/philosophy_v0_1.md").to_string(),
            );
        resources.insert("integrity://philosophy".to_string(), philosophy_content);

        // constraints_v0_1.md
        let constraints_content =
            std::fs::read_to_string(format!("{}/constraints_v0_1.md", base_path)).unwrap_or_else(
                |_| include_str!("../../../.cursor/rules/constraints_v0_1.md").to_string(),
            );
        resources.insert("integrity://constraints".to_string(), constraints_content);

        Ok(Self { resources })
    }

    /// リクエストを処理
    fn handle_request(&self, request: &JsonRpcRequest) -> JsonRpcResponse {
        match request.method.as_str() {
            "initialize" => self.handle_initialize(request),
            "notifications/initialized" => {
                // 通知なのでレスポンスは不要（空のレスポンスを返す）
                JsonRpcResponse::success(request.id.clone(), serde_json::json!({}))
            }
            "resources/list" => self.handle_resources_list(request),
            "resources/read" => self.handle_resources_read(request),
            "tools/list" => self.handle_tools_list(request),
            "tools/call" => self.handle_tools_call(request),
            _ => JsonRpcResponse::error(
                request.id.clone(),
                -32601,
                format!("Method not found: {}", request.method),
            ),
        }
    }

    /// initialize ハンドラ
    fn handle_initialize(&self, request: &JsonRpcRequest) -> JsonRpcResponse {
        JsonRpcResponse::success(
            request.id.clone(),
            serde_json::json!({
                "protocolVersion": PROTOCOL_VERSION,
                "capabilities": {
                    "resources": {
                        "subscribe": false,
                        "listChanged": false
                    },
                    "tools": {
                        "listChanged": false
                    }
                },
                "serverInfo": {
                    "name": "integrity_mcp",
                    "version": SERVER_VERSION
                }
            }),
        )
    }

    /// resources/list ハンドラ
    fn handle_resources_list(&self, request: &JsonRpcRequest) -> JsonRpcResponse {
        let resources: Vec<serde_json::Value> = vec![
            serde_json::json!({
                "uri": "integrity://project",
                "name": "project_v0_1.md",
                "description": "プロジェクト仕様書",
                "mimeType": "text/markdown"
            }),
            serde_json::json!({
                "uri": "integrity://philosophy",
                "name": "philosophy_v0_1.md",
                "description": "設計哲学",
                "mimeType": "text/markdown"
            }),
            serde_json::json!({
                "uri": "integrity://constraints",
                "name": "constraints_v0_1.md",
                "description": "ハード制約一覧",
                "mimeType": "text/markdown"
            }),
        ];

        JsonRpcResponse::success(
            request.id.clone(),
            serde_json::json!({
                "resources": resources
            }),
        )
    }

    /// resources/read ハンドラ
    fn handle_resources_read(&self, request: &JsonRpcRequest) -> JsonRpcResponse {
        let uri = request.params.get("uri").and_then(|v| v.as_str());

        match uri {
            Some(uri) => {
                if let Some(content) = self.resources.get(uri) {
                    JsonRpcResponse::success(
                        request.id.clone(),
                        serde_json::json!({
                            "contents": [{
                                "uri": uri,
                                "mimeType": "text/markdown",
                                "text": content
                            }]
                        }),
                    )
                } else {
                    JsonRpcResponse::error(
                        request.id.clone(),
                        -32602,
                        format!("Resource not found: {}", uri),
                    )
                }
            }
            None => JsonRpcResponse::error(
                request.id.clone(),
                -32602,
                "Missing required parameter: uri".to_string(),
            ),
        }
    }

    /// tools/list ハンドラ
    fn handle_tools_list(&self, request: &JsonRpcRequest) -> JsonRpcResponse {
        JsonRpcResponse::success(
            request.id.clone(),
            serde_json::json!({
                "tools": [{
                    "name": "check_constraints",
                    "description": "テキストのハード制約違反を検出します。制約①〜⑦に対してチェックを行い、違反があればレポートを返します。",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "input": {
                                "type": "string",
                                "description": "チェック対象のテキスト"
                            },
                            "format": {
                                "type": "string",
                                "enum": ["json", "human"],
                                "description": "出力フォーマット（デフォルト: human）"
                            }
                        },
                        "required": ["input"]
                    }
                }]
            }),
        )
    }

    /// tools/call ハンドラ
    fn handle_tools_call(&self, request: &JsonRpcRequest) -> JsonRpcResponse {
        let tool_name = request.params.get("name").and_then(|v| v.as_str());
        let arguments = request.params.get("arguments");

        match tool_name {
            Some("check_constraints") => {
                self.handle_check_constraints(request.id.clone(), arguments)
            }
            Some(name) => JsonRpcResponse::error(
                request.id.clone(),
                -32602,
                format!("Unknown tool: {}", name),
            ),
            None => JsonRpcResponse::error(
                request.id.clone(),
                -32602,
                "Missing required parameter: name".to_string(),
            ),
        }
    }

    /// check_constraints ツールハンドラ
    fn handle_check_constraints(
        &self,
        id: Option<serde_json::Value>,
        arguments: Option<&serde_json::Value>,
    ) -> JsonRpcResponse {
        let args = match arguments {
            Some(args) => args,
            None => {
                return JsonRpcResponse::error(id, -32602, "Missing arguments".to_string());
            }
        };

        let input = match args.get("input").and_then(|v| v.as_str()) {
            Some(input) => input,
            None => {
                return JsonRpcResponse::error(
                    id,
                    -32602,
                    "Missing required argument: input".to_string(),
                );
            }
        };

        // 入力の検証
        if input.trim().is_empty() {
            return JsonRpcResponse::error(id, -32602, "Input is empty".to_string());
        }

        if input.len() > MAX_INPUT_SIZE {
            return JsonRpcResponse::error(
                id,
                -32602,
                format!(
                    "Input too large (max: {} bytes, actual: {} bytes)",
                    MAX_INPUT_SIZE,
                    input.len()
                ),
            );
        }

        let format = args
            .get("format")
            .and_then(|v| v.as_str())
            .unwrap_or("human");

        // チェック実行
        let report = integrity_rules::check_text(input);

        // フォーマット
        let content = match format {
            "json" => match report.to_json() {
                Ok(json) => json,
                Err(e) => {
                    return JsonRpcResponse::error(id, -32603, format!("JSON error: {}", e));
                }
            },
            _ => report.to_human_readable(),
        };

        JsonRpcResponse::success(
            id,
            serde_json::json!({
                "content": [{
                    "type": "text",
                    "text": content
                }],
                "isError": report.has_violations()
            }),
        )
    }
}

fn main() -> Result<()> {
    let server = McpServer::new()?;

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lock().lines() {
        let line = line.context("Failed to read line")?;

        if line.trim().is_empty() {
            continue;
        }

        let request: JsonRpcRequest = match serde_json::from_str(&line) {
            Ok(req) => req,
            Err(e) => {
                let error_response =
                    JsonRpcResponse::error(None, -32700, format!("Parse error: {}", e));
                let output = serde_json::to_string(&error_response)?;
                writeln!(stdout, "{}", output)?;
                stdout.flush()?;
                continue;
            }
        };

        // 通知の場合はレスポンスを返さない
        if request.method.starts_with("notifications/") && request.id.is_none() {
            continue;
        }

        let response = server.handle_request(&request);
        let output = serde_json::to_string(&response)?;
        writeln!(stdout, "{}", output)?;
        stdout.flush()?;
    }

    Ok(())
}

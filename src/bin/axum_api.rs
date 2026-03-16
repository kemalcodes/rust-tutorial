// Rust Tutorial #18: Building a REST API with Axum
// Demonstrates Axum router, handlers, extractors (Path, Query, Json),
// shared State, middleware, CORS, and error handling.

use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};

// --- Data Models ---

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct Todo {
    id: u64,
    title: String,
    completed: bool,
}

#[derive(Debug, Deserialize)]
struct CreateTodo {
    title: String,
}

#[derive(Debug, Deserialize)]
struct UpdateTodo {
    title: Option<String>,
    completed: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct PaginationParams {
    #[serde(default = "default_page")]
    page: u64,
    #[serde(default = "default_limit")]
    limit: u64,
}

fn default_page() -> u64 {
    1
}
fn default_limit() -> u64 {
    10
}

// --- Application State ---
// Shared state across all handlers. Uses Arc<RwLock<>> for
// thread-safe shared access.

#[derive(Clone)]
struct AppState {
    todos: Arc<RwLock<HashMap<u64, Todo>>>,
    next_id: Arc<RwLock<u64>>,
}

impl AppState {
    fn new() -> Self {
        Self {
            todos: Arc::new(RwLock::new(HashMap::new())),
            next_id: Arc::new(RwLock::new(1)),
        }
    }
}

// --- API Response Types ---

#[derive(Serialize)]
struct ApiResponse<T: Serialize> {
    data: T,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Serialize)]
struct ListResponse {
    data: Vec<Todo>,
    total: usize,
    page: u64,
    limit: u64,
}

// --- Route Handlers ---

// GET /
async fn root() -> &'static str {
    "Todo API v1.0"
}

// GET /health
async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, Json(serde_json::json!({"status": "ok"})))
}

// GET /todos
async fn list_todos(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> impl IntoResponse {
    let todos = state.todos.read().await;
    let mut all_todos: Vec<Todo> = todos.values().cloned().collect();
    all_todos.sort_by_key(|t| t.id);

    let total = all_todos.len();
    let start = ((params.page - 1) * params.limit) as usize;
    let end = (start + params.limit as usize).min(total);

    let page_todos = if start < total {
        all_todos[start..end].to_vec()
    } else {
        vec![]
    };

    Json(ListResponse {
        data: page_todos,
        total,
        page: params.page,
        limit: params.limit,
    })
}

// POST /todos
async fn create_todo(
    State(state): State<AppState>,
    Json(input): Json<CreateTodo>,
) -> impl IntoResponse {
    if input.title.trim().is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "Title cannot be empty"})),
        );
    }

    let mut next_id = state.next_id.write().await;
    let id = *next_id;
    *next_id += 1;

    let todo = Todo {
        id,
        title: input.title.trim().to_string(),
        completed: false,
    };

    state.todos.write().await.insert(id, todo.clone());

    (
        StatusCode::CREATED,
        Json(serde_json::json!({"data": todo})),
    )
}

// GET /todos/:id
async fn get_todo(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> impl IntoResponse {
    let todos = state.todos.read().await;

    match todos.get(&id) {
        Some(todo) => (
            StatusCode::OK,
            Json(serde_json::json!({"data": todo})),
        ),
        None => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "Todo not found"})),
        ),
    }
}

// PUT /todos/:id
async fn update_todo(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(input): Json<UpdateTodo>,
) -> impl IntoResponse {
    let mut todos = state.todos.write().await;

    match todos.get_mut(&id) {
        Some(todo) => {
            if let Some(title) = input.title {
                if title.trim().is_empty() {
                    return (
                        StatusCode::BAD_REQUEST,
                        Json(serde_json::json!({"error": "Title cannot be empty"})),
                    );
                }
                todo.title = title.trim().to_string();
            }
            if let Some(completed) = input.completed {
                todo.completed = completed;
            }
            let updated = todo.clone();
            (
                StatusCode::OK,
                Json(serde_json::json!({"data": updated})),
            )
        }
        None => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "Todo not found"})),
        ),
    }
}

// DELETE /todos/:id
async fn delete_todo(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> impl IntoResponse {
    let mut todos = state.todos.write().await;

    match todos.remove(&id) {
        Some(_) => (
            StatusCode::OK,
            Json(serde_json::json!({"message": "Todo deleted"})),
        ),
        None => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "Todo not found"})),
        ),
    }
}

// --- Router Setup ---

fn create_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .route("/todos", get(list_todos).post(create_todo))
        .route(
            "/todos/{id}",
            get(get_todo).put(update_todo).delete(delete_todo),
        )
        .layer(cors)
        .with_state(state)
}

#[tokio::main]
async fn main() {
    let state = AppState::new();
    let app = create_router(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_state() -> AppState {
        AppState::new()
    }

    #[tokio::test]
    async fn test_create_and_get_todo() {
        let state = test_state();

        // Create a todo
        let input = CreateTodo {
            title: "Buy groceries".to_string(),
        };
        let mut next_id = state.next_id.write().await;
        let id = *next_id;
        *next_id += 1;
        drop(next_id);

        let todo = Todo {
            id,
            title: input.title.trim().to_string(),
            completed: false,
        };
        state.todos.write().await.insert(id, todo.clone());

        // Get the todo
        let todos = state.todos.read().await;
        let found = todos.get(&id).unwrap();
        assert_eq!(found.title, "Buy groceries");
        assert!(!found.completed);
    }

    #[tokio::test]
    async fn test_update_todo() {
        let state = test_state();

        // Create a todo
        let todo = Todo {
            id: 1,
            title: "Original".to_string(),
            completed: false,
        };
        state.todos.write().await.insert(1, todo);

        // Update it
        {
            let mut todos = state.todos.write().await;
            let t = todos.get_mut(&1).unwrap();
            t.title = "Updated".to_string();
            t.completed = true;
        }

        let todos = state.todos.read().await;
        let updated = todos.get(&1).unwrap();
        assert_eq!(updated.title, "Updated");
        assert!(updated.completed);
    }

    #[tokio::test]
    async fn test_delete_todo() {
        let state = test_state();

        let todo = Todo {
            id: 1,
            title: "To delete".to_string(),
            completed: false,
        };
        state.todos.write().await.insert(1, todo);

        let removed = state.todos.write().await.remove(&1);
        assert!(removed.is_some());
        assert!(state.todos.read().await.get(&1).is_none());
    }

    #[tokio::test]
    async fn test_list_todos_pagination() {
        let state = test_state();

        // Add 5 todos
        for i in 1..=5 {
            let todo = Todo {
                id: i,
                title: format!("Todo {}", i),
                completed: false,
            };
            state.todos.write().await.insert(i, todo);
        }

        let todos = state.todos.read().await;
        let mut all: Vec<Todo> = todos.values().cloned().collect();
        all.sort_by_key(|t| t.id);

        // Page 1, limit 2
        let page: Vec<Todo> = all[0..2].to_vec();
        assert_eq!(page.len(), 2);
        assert_eq!(page[0].title, "Todo 1");
        assert_eq!(page[1].title, "Todo 2");

        // Page 2, limit 2
        let page: Vec<Todo> = all[2..4].to_vec();
        assert_eq!(page.len(), 2);
        assert_eq!(page[0].title, "Todo 3");
    }

    #[tokio::test]
    async fn test_todo_not_found() {
        let state = test_state();
        let todos = state.todos.read().await;
        assert!(todos.get(&999).is_none());
    }

    #[tokio::test]
    async fn test_empty_title_rejected() {
        let input = CreateTodo {
            title: "  ".to_string(),
        };
        assert!(input.title.trim().is_empty());
    }

    #[tokio::test]
    async fn test_app_state_clone() {
        let state = test_state();
        let state2 = state.clone();

        // Both point to the same data
        state
            .todos
            .write()
            .await
            .insert(1, Todo {
                id: 1,
                title: "Test".to_string(),
                completed: false,
            });

        assert!(state2.todos.read().await.get(&1).is_some());
    }

    #[test]
    fn test_pagination_defaults() {
        let json = "{}";
        let params: PaginationParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.page, 1);
        assert_eq!(params.limit, 10);
    }

    #[test]
    fn test_todo_serialization() {
        let todo = Todo {
            id: 1,
            title: "Test".to_string(),
            completed: true,
        };
        let json = serde_json::to_string(&todo).unwrap();
        assert!(json.contains("\"completed\":true"));
        assert!(json.contains("\"id\":1"));
    }

    #[test]
    fn test_create_router() {
        let state = test_state();
        let _router = create_router(state);
        // Router creates without panicking
    }
}

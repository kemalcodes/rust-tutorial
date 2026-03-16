// Rust Tutorial #19: Database with SQLx
// Demonstrates SQLx setup, CRUD operations with SQLite,
// connection pool, queries, and error handling.
// Uses runtime-checked queries (not compile-time) for simplicity.

use serde::{Deserialize, Serialize};
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use sqlx::{FromRow, Row};

// --- Data Models ---

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
struct Todo {
    id: i64,
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

// --- Database Setup ---

async fn create_pool(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;
    Ok(pool)
}

async fn run_migrations(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            completed BOOLEAN NOT NULL DEFAULT FALSE
        )",
    )
    .execute(pool)
    .await?;
    Ok(())
}

// --- CRUD Operations ---

async fn insert_todo(pool: &SqlitePool, title: &str) -> Result<Todo, sqlx::Error> {
    let result = sqlx::query("INSERT INTO todos (title, completed) VALUES (?, FALSE)")
        .bind(title)
        .execute(pool)
        .await?;

    let id = result.last_insert_rowid();

    let todo = sqlx::query_as::<_, Todo>("SELECT id, title, completed FROM todos WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await?;

    Ok(todo)
}

async fn get_todo_by_id(pool: &SqlitePool, id: i64) -> Result<Option<Todo>, sqlx::Error> {
    let todo = sqlx::query_as::<_, Todo>("SELECT id, title, completed FROM todos WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?;
    Ok(todo)
}

async fn get_all_todos(pool: &SqlitePool) -> Result<Vec<Todo>, sqlx::Error> {
    let todos =
        sqlx::query_as::<_, Todo>("SELECT id, title, completed FROM todos ORDER BY id ASC")
            .fetch_all(pool)
            .await?;
    Ok(todos)
}

async fn get_todos_paginated(
    pool: &SqlitePool,
    limit: i64,
    offset: i64,
) -> Result<Vec<Todo>, sqlx::Error> {
    let todos = sqlx::query_as::<_, Todo>(
        "SELECT id, title, completed FROM todos ORDER BY id ASC LIMIT ? OFFSET ?",
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;
    Ok(todos)
}

async fn count_todos(pool: &SqlitePool) -> Result<i64, sqlx::Error> {
    let row = sqlx::query("SELECT COUNT(*) as count FROM todos")
        .fetch_one(pool)
        .await?;
    let count: i64 = row.get("count");
    Ok(count)
}

async fn update_todo_title(
    pool: &SqlitePool,
    id: i64,
    title: &str,
) -> Result<Option<Todo>, sqlx::Error> {
    let result = sqlx::query("UPDATE todos SET title = ? WHERE id = ?")
        .bind(title)
        .bind(id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Ok(None);
    }

    get_todo_by_id(pool, id).await
}

async fn toggle_todo_completed(
    pool: &SqlitePool,
    id: i64,
) -> Result<Option<Todo>, sqlx::Error> {
    let result = sqlx::query("UPDATE todos SET completed = NOT completed WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Ok(None);
    }

    get_todo_by_id(pool, id).await
}

async fn delete_todo(pool: &SqlitePool, id: i64) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM todos WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected() > 0)
}

async fn delete_completed_todos(pool: &SqlitePool) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("DELETE FROM todos WHERE completed = TRUE")
        .execute(pool)
        .await?;
    Ok(result.rows_affected())
}

// --- Search and Filter ---

async fn search_todos(pool: &SqlitePool, query: &str) -> Result<Vec<Todo>, sqlx::Error> {
    let pattern = format!("%{}%", query);
    let todos = sqlx::query_as::<_, Todo>(
        "SELECT id, title, completed FROM todos WHERE title LIKE ? ORDER BY id ASC",
    )
    .bind(pattern)
    .fetch_all(pool)
    .await?;
    Ok(todos)
}

async fn get_completed_todos(pool: &SqlitePool) -> Result<Vec<Todo>, sqlx::Error> {
    let todos = sqlx::query_as::<_, Todo>(
        "SELECT id, title, completed FROM todos WHERE completed = TRUE ORDER BY id ASC",
    )
    .fetch_all(pool)
    .await?;
    Ok(todos)
}

async fn get_pending_todos(pool: &SqlitePool) -> Result<Vec<Todo>, sqlx::Error> {
    let todos = sqlx::query_as::<_, Todo>(
        "SELECT id, title, completed FROM todos WHERE completed = FALSE ORDER BY id ASC",
    )
    .fetch_all(pool)
    .await?;
    Ok(todos)
}

// --- Transaction Example ---

async fn create_multiple_todos(
    pool: &SqlitePool,
    titles: &[&str],
) -> Result<Vec<Todo>, sqlx::Error> {
    let mut tx = pool.begin().await?;
    let mut todos = vec![];

    for title in titles {
        let result = sqlx::query("INSERT INTO todos (title, completed) VALUES (?, FALSE)")
            .bind(title)
            .execute(&mut *tx)
            .await?;

        let id = result.last_insert_rowid();
        let todo =
            sqlx::query_as::<_, Todo>("SELECT id, title, completed FROM todos WHERE id = ?")
                .bind(id)
                .fetch_one(&mut *tx)
                .await?;
        todos.push(todo);
    }

    tx.commit().await?;
    Ok(todos)
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Use in-memory SQLite for demo
    let pool = create_pool("sqlite::memory:").await?;
    run_migrations(&pool).await?;

    // Create todos
    let todo1 = insert_todo(&pool, "Buy groceries").await?;
    println!("Created: {:?}", todo1);

    let todo2 = insert_todo(&pool, "Write Rust tutorial").await?;
    println!("Created: {:?}", todo2);

    let todo3 = insert_todo(&pool, "Read a book").await?;
    println!("Created: {:?}", todo3);

    // List all
    let all = get_all_todos(&pool).await?;
    println!("\nAll todos ({}):", all.len());
    for todo in &all {
        println!("  {:?}", todo);
    }

    // Get by ID
    let found = get_todo_by_id(&pool, 1).await?;
    println!("\nFound by ID 1: {:?}", found);

    // Update title
    let updated = update_todo_title(&pool, 1, "Buy organic groceries").await?;
    println!("Updated: {:?}", updated);

    // Toggle completed
    let toggled = toggle_todo_completed(&pool, 2).await?;
    println!("Toggled: {:?}", toggled);

    // Search
    let results = search_todos(&pool, "Rust").await?;
    println!("\nSearch 'Rust': {:?}", results);

    // Count
    let count = count_todos(&pool).await?;
    println!("Total todos: {}", count);

    // Get completed
    let completed = get_completed_todos(&pool).await?;
    println!("Completed: {:?}", completed);

    // Delete
    let deleted = delete_todo(&pool, 3).await?;
    println!("Deleted ID 3: {}", deleted);

    // Transaction
    let batch = create_multiple_todos(&pool, &["Task A", "Task B", "Task C"]).await?;
    println!("\nBatch created:");
    for todo in &batch {
        println!("  {:?}", todo);
    }

    // Final count
    let count = count_todos(&pool).await?;
    println!("\nFinal count: {}", count);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn setup_test_db() -> SqlitePool {
        let pool = create_pool("sqlite::memory:").await.unwrap();
        run_migrations(&pool).await.unwrap();
        pool
    }

    #[tokio::test]
    async fn test_create_and_get() {
        let pool = setup_test_db().await;

        let todo = insert_todo(&pool, "Test todo").await.unwrap();
        assert_eq!(todo.title, "Test todo");
        assert!(!todo.completed);

        let found = get_todo_by_id(&pool, todo.id).await.unwrap();
        assert_eq!(found, Some(todo));
    }

    #[tokio::test]
    async fn test_get_nonexistent() {
        let pool = setup_test_db().await;

        let found = get_todo_by_id(&pool, 999).await.unwrap();
        assert_eq!(found, None);
    }

    #[tokio::test]
    async fn test_get_all() {
        let pool = setup_test_db().await;

        insert_todo(&pool, "First").await.unwrap();
        insert_todo(&pool, "Second").await.unwrap();
        insert_todo(&pool, "Third").await.unwrap();

        let all = get_all_todos(&pool).await.unwrap();
        assert_eq!(all.len(), 3);
        assert_eq!(all[0].title, "First");
        assert_eq!(all[1].title, "Second");
        assert_eq!(all[2].title, "Third");
    }

    #[tokio::test]
    async fn test_update_title() {
        let pool = setup_test_db().await;

        let todo = insert_todo(&pool, "Original").await.unwrap();
        let updated = update_todo_title(&pool, todo.id, "Updated").await.unwrap();
        assert_eq!(updated.unwrap().title, "Updated");
    }

    #[tokio::test]
    async fn test_update_nonexistent() {
        let pool = setup_test_db().await;

        let result = update_todo_title(&pool, 999, "Nope").await.unwrap();
        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn test_toggle_completed() {
        let pool = setup_test_db().await;

        let todo = insert_todo(&pool, "Toggle me").await.unwrap();
        assert!(!todo.completed);

        let toggled = toggle_todo_completed(&pool, todo.id).await.unwrap().unwrap();
        assert!(toggled.completed);

        let toggled_back = toggle_todo_completed(&pool, todo.id).await.unwrap().unwrap();
        assert!(!toggled_back.completed);
    }

    #[tokio::test]
    async fn test_delete() {
        let pool = setup_test_db().await;

        let todo = insert_todo(&pool, "Delete me").await.unwrap();
        let deleted = delete_todo(&pool, todo.id).await.unwrap();
        assert!(deleted);

        let found = get_todo_by_id(&pool, todo.id).await.unwrap();
        assert_eq!(found, None);
    }

    #[tokio::test]
    async fn test_delete_nonexistent() {
        let pool = setup_test_db().await;

        let deleted = delete_todo(&pool, 999).await.unwrap();
        assert!(!deleted);
    }

    #[tokio::test]
    async fn test_search() {
        let pool = setup_test_db().await;

        insert_todo(&pool, "Buy groceries").await.unwrap();
        insert_todo(&pool, "Write Rust code").await.unwrap();
        insert_todo(&pool, "Buy milk").await.unwrap();

        let results = search_todos(&pool, "Buy").await.unwrap();
        assert_eq!(results.len(), 2);

        let results = search_todos(&pool, "Rust").await.unwrap();
        assert_eq!(results.len(), 1);

        let results = search_todos(&pool, "nothing").await.unwrap();
        assert_eq!(results.len(), 0);
    }

    #[tokio::test]
    async fn test_completed_and_pending() {
        let pool = setup_test_db().await;

        let t1 = insert_todo(&pool, "Done task").await.unwrap();
        insert_todo(&pool, "Pending task").await.unwrap();
        toggle_todo_completed(&pool, t1.id).await.unwrap();

        let completed = get_completed_todos(&pool).await.unwrap();
        assert_eq!(completed.len(), 1);
        assert_eq!(completed[0].title, "Done task");

        let pending = get_pending_todos(&pool).await.unwrap();
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].title, "Pending task");
    }

    #[tokio::test]
    async fn test_count() {
        let pool = setup_test_db().await;

        assert_eq!(count_todos(&pool).await.unwrap(), 0);

        insert_todo(&pool, "One").await.unwrap();
        insert_todo(&pool, "Two").await.unwrap();

        assert_eq!(count_todos(&pool).await.unwrap(), 2);
    }

    #[tokio::test]
    async fn test_delete_completed() {
        let pool = setup_test_db().await;

        let t1 = insert_todo(&pool, "Done 1").await.unwrap();
        let t2 = insert_todo(&pool, "Done 2").await.unwrap();
        insert_todo(&pool, "Pending").await.unwrap();

        toggle_todo_completed(&pool, t1.id).await.unwrap();
        toggle_todo_completed(&pool, t2.id).await.unwrap();

        let deleted = delete_completed_todos(&pool).await.unwrap();
        assert_eq!(deleted, 2);

        let all = get_all_todos(&pool).await.unwrap();
        assert_eq!(all.len(), 1);
        assert_eq!(all[0].title, "Pending");
    }

    #[tokio::test]
    async fn test_transaction() {
        let pool = setup_test_db().await;

        let todos = create_multiple_todos(&pool, &["A", "B", "C"]).await.unwrap();
        assert_eq!(todos.len(), 3);
        assert_eq!(todos[0].title, "A");
        assert_eq!(todos[1].title, "B");
        assert_eq!(todos[2].title, "C");

        let count = count_todos(&pool).await.unwrap();
        assert_eq!(count, 3);
    }
}

use crate::db::models::PromptDirectory;

// --- Prompt Directory CRUD ---
    pub async fn create_directory(&self, name: &str, parent_id: Option<i64>) -> Result<i64> {
        let rec = sqlx::query!(
            r#"INSERT INTO prompt_directory (name, parent_id) VALUES (?, ?)"#,
            name,
            parent_id
        )
        .execute(&self.pool)
        .await?;
        Ok(rec.last_insert_rowid())
    }

    pub async fn get_directory(&self, id: i64) -> Result<Option<PromptDirectory>> {
        let rec = sqlx::query_as!(PromptDirectory,
            r#"SELECT id, name, parent_id, created_at, updated_at FROM prompt_directory WHERE id = ?"#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(rec)
    }

    pub async fn list_directories(&self, parent_id: Option<i64>) -> Result<Vec<PromptDirectory>> {
        let recs = sqlx::query_as!(PromptDirectory,
            r#"SELECT id, name, parent_id, created_at, updated_at FROM prompt_directory WHERE parent_id IS ?"#,
            parent_id
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(recs)
    }

    pub async fn update_directory(&self, id: i64, name: &str, parent_id: Option<i64>) -> Result<bool> {
        let rows = sqlx::query!(
            r#"UPDATE prompt_directory SET name = ?, parent_id = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?"#,
            name, parent_id, id
        )
        .execute(&self.pool)
        .await?
        .rows_affected();
        Ok(rows > 0)
    }

    pub async fn delete_directory(&self, id: i64) -> Result<bool> {
        let rows = sqlx::query!(
            r#"DELETE FROM prompt_directory WHERE id = ?"#,
            id
        )
        .execute(&self.pool)
        .await?
        .rows_affected();
        Ok(rows > 0)
    }

// All table names below should be prompt_directory (singular)

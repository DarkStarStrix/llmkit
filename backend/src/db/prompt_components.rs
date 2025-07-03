use crate::db::models::PromptComponent;
use regex::Regex;

// --- Prompt Component CRUD ---
    pub async fn create_component(&self, name: &str, content: &str, description: Option<&str>) -> Result<i64> {
        let rec = sqlx::query!(
            r#"INSERT INTO prompt_component (name, content, description) VALUES (?, ?, ?)"#,
            name, content, description
        )
        .execute(&self.pool)
        .await?;
        Ok(rec.last_insert_rowid())
    }

    pub async fn get_component(&self, id: i64) -> Result<Option<PromptComponent>> {
        let rec = sqlx::query_as!(PromptComponent,
            r#"SELECT id, name, content, description, created_at, updated_at FROM prompt_component WHERE id = ?"#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(rec)
    }

    pub async fn list_components(&self) -> Result<Vec<PromptComponent>> {
        let recs = sqlx::query_as!(PromptComponent,
            r#"SELECT id, name, content, description, created_at, updated_at FROM prompt_component ORDER BY created_at DESC"#
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(recs)
    }

    pub async fn update_component(&self, id: i64, name: &str, content: &str, description: Option<&str>) -> Result<bool> {
        let rows = sqlx::query!(
            r#"UPDATE prompt_component SET name = ?, content = ?, description = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?"#,
            name, content, description, id
        )
        .execute(&self.pool)
        .await?
        .rows_affected();
        Ok(rows > 0)
    }

    pub async fn delete_component(&self, id: i64) -> Result<bool> {
        let rows = sqlx::query!(
            r#"DELETE FROM prompt_component WHERE id = ?"#,
            id
        )
        .execute(&self.pool)
        .await?
        .rows_affected();
        Ok(rows > 0)
    }

    // Utility: Recursively resolve {{component:component_name}} in prompt content
    pub async fn resolve_components_in_text(&self, text: &str) -> Result<String> {
        let re = Regex::new(r"\{\{component:([a-zA-Z0-9_\- ]+)}}")?;
        let mut resolved = text.to_string();
        let mut changed = true;
        while changed {
            changed = false;
            let mut new_text = resolved.clone();
            for cap in re.captures_iter(&resolved) {
                let comp_name = &cap[1];
                let comp = sqlx::query!(
                    r#"SELECT content FROM prompt_component WHERE name = ? LIMIT 1"#,
                    comp_name
                )
                .fetch_optional(&self.pool)
                .await?;
                if let Some(row) = comp {
                    let comp_content = row.content;
                    new_text = new_text.replace(&cap[0], &comp_content);
                    changed = true;
                }
            }
            resolved = new_text;
        }
        Ok(resolved)
    }

    // Wrap get_prompt to resolve components in system/user fields
    pub async fn get_prompt_with_components(&self, id: i64) -> Result<PromptRowWithModel> {
        let mut prompt = self.get_prompt(id).await?;
        if let Some(system) = &prompt.system {
            prompt.system = Some(self.resolve_components_in_text(system).await?);
        }
        if let Some(user) = &prompt.user {
            prompt.user = Some(self.resolve_components_in_text(user).await?);
        }
        Ok(prompt)
    }
}

fn generate_diff(text1: &str, text2: &str) -> String {
    let mut diff_string = String::new();
    let differences = lines(text1, text2);

    for difference in differences {
        match difference {
            DiffResult::Left(l) => diff_string.push_str(&format!("-{}\n", l)),
            DiffResult::Right(r) => diff_string.push_str(&format!("+{}\n", r)),
            _ => {}
        }
    }

    diff_string
}

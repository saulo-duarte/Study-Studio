use rusqlite::{Connection, Result, params};
use rusqlite::OptionalExtension;
use crate::db::models::document::Document;
use serde_json;
use chrono::{DateTime, Utc};

pub struct DocumentRepository<'a> {
    conn: &'a Connection,
}

impl<'a> DocumentRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn create(&self, document: &Document) -> Result<(), String> {
        let query = "
            INSERT INTO documents (
                title, 
                original_filename, 
                stored_filename, 
                file_path, 
                file_size, 
                mime_type, 
                hash, 
                page_count, 
                created_at, 
                last_accessed, 
                thumbnail_path, 
                tags
            ) 
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)";
    
        self.conn
            .execute(
                query, 
                params![
                    document.title,
                    document.original_filename,
                    document.stored_filename,
                    document.file_path,
                    document.file_size,
                    document.mime_type,
                    document.hash,
                    document.page_count,
                    document.created_at.map(|dt| dt.to_rfc3339()),
                    document.last_accessed.map(|dt| dt.to_rfc3339()),
                    document.thumbnail_path,
                    document.tags.as_ref().map(|t| serde_json::to_string(t).unwrap_or_default()),
                ]
            )
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn get_by_id(&self, id: i32) -> Result<Option<Document>, String> {
        let query = "
            SELECT 
                id, title, original_filename, stored_filename, file_path, 
                file_size, mime_type, hash, page_count, created_at, 
                last_accessed, thumbnail_path, tags
            FROM documents 
            WHERE id = ?1";

        let result = self.conn.query_row(
            query, 
            params![id], 
            |row| {
                Ok(Document {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    original_filename: row.get(2)?,
                    stored_filename: row.get(3)?,
                    file_path: row.get(4)?,
                    file_size: row.get(5)?,
                    mime_type: row.get(6)?,
                    hash: row.get(7)?,
                    page_count: row.get(8)?,
                    created_at: row.get::<_, Option<String>>(9)?
                        .map(|dt_str| dt_str.parse::<DateTime<Utc>>().unwrap()),
                    last_accessed: row.get::<_, Option<String>>(9)?
                    .map(|dt_str| dt_str.parse::<DateTime<Utc>>().unwrap()),
                    thumbnail_path: row.get(11)?,
                    tags: row.get::<_, Option<String>>(12)?
                        .map(|tags_str| serde_json::from_str(&tags_str).unwrap_or_default()),
                })
            }
        )
        .optional()
        .map_err(|e| e.to_string())?;

        Ok(result)
    }

    pub fn update(&self, document: &Document) -> Result<(), String> {
        let query = "pub use user_available_day_repository::*;

            UPDATE documents 
            SET title = ?1, 
                original_filename = ?2, 
                stored_filename = ?3, 
                file_path = ?4, 
                file_size = ?5, 
                mime_type = ?6, 
                hash = ?7, 
                page_count = ?8, 
                created_at = ?9, 
                last_accessed = ?10, 
                thumbnail_path = ?11, 
                tags = ?12 
            WHERE id = ?13";

        self.conn
            .execute(
                query, 
                params![
                    document.title,
                    document.original_filename,
                    document.stored_filename,
                    document.file_path,
                    document.file_size,
                    document.mime_type,
                    document.hash,
                    document.page_count,
                    document.created_at.map(|dt| dt.to_rfc3339()),
                    document.last_accessed.map(|dt| dt.to_rfc3339()),
                    document.thumbnail_path,
                    document.tags.as_ref().map(|t| serde_json::to_string(t).unwrap_or_default()),
                    document.id
                ]
            )
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn delete(&self, id: i32) -> Result<(), String> {
        let query = "DELETE FROM documents WHERE id = ?1";
        self.conn
            .execute(query, params![id])
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn get_all(&self) -> Result<Vec<Document>, String> {
        let query = "
            SELECT 
                id, title, original_filename, stored_filename, file_path, 
                file_size, mime_type, hash, page_count, created_at, 
                last_accessed, thumbnail_path, tags 
            FROM documents";

        let mut stmt = self.conn.prepare(query).map_err(|e| e.to_string())?;

        let document_iter = stmt.query_map([], |row| {
            Ok(Document {
                id: row.get(0)?,
                title: row.get(1)?,
                original_filename: row.get(2)?,
                stored_filename: row.get(3)?,
                file_path: row.get(4)?,
                file_size: row.get(5)?,
                mime_type: row.get(6)?,
                hash: row.get(7)?,
                page_count: row.get(8)?,
                created_at: row.get::<_, Option<String>>(9)?
                    .map(|dt_str| dt_str.parse::<DateTime<Utc>>().unwrap()),
                last_accessed: row.get::<_, Option<String>>(9)?
                    .map(|dt_str| dt_str.parse::<DateTime<Utc>>().unwrap()),
                thumbnail_path: row.get(11)?,
                tags: row.get::<_, Option<String>>(12)?
                    .map(|tags_str| serde_json::from_str(&tags_str).unwrap_or_default()),
            })
        })
        .map_err(|e| e.to_string())?;

        document_iter.collect::<Result<_, _>>()
            .map_err(|e| e.to_string())
    }
}
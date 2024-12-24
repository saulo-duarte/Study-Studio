use rusqlite::{Connection, Result, params, OptionalExtension};
use crate::db::models::tag::Tag;

pub struct TagRepository<'a> {
    conn: &'a Connection,
}

impl<'a> TagRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn create(&self, tag: &Tag) -> Result<(), String> {
        let sql = "
            INSERT INTO tags (
                title, 
                color, 
                description, 
                created_at, 
                icon
            ) 
            VALUES (?, ?, ?, ?, ?)
            RETURNING *";

        self.conn
            .execute(sql, params![
                &tag.title,
                &tag.color,
                &tag.description,
                &tag.created_at,
                &tag.icon,
            ])
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn get_by_id(&self, id: i64) -> Result<Option<Tag>, String> {
        let sql = "
            SELECT id, title, color, description, created_at, icon 
            FROM tags 
            WHERE id = ?";

        self.conn.query_row(sql, params![id], |row| {
            Ok(Tag {
                id: row.get(0)?,
                title: row.get(1)?,
                color: row.get(2)?,
                description: row.get(3).ok(),
                created_at: row.get(4)?,
                icon: row.get(5).ok(),
            })
        }).optional()
        .map_err(|e| e.to_string())
    }

    pub fn update(&self, tag: &Tag) -> Result<(), String> {
        let sql = "
            UPDATE tags 
            SET title = ?, color = ?, description = ?, created_at = ?, icon = ? 
            WHERE id = ?";

        self.conn
            .execute(sql, params![
                &tag.title,
                &tag.color,
                &tag.description,
                &tag.created_at,
                &tag.icon,
                &tag.id
            ])
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn delete(&self, id: i64) -> Result<(), String> {
        let sql = "DELETE FROM tags WHERE id = ?";

        self.conn
            .execute(sql, params![id])
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn get_all(&self) -> Result<Vec<Tag>, String> {
        let sql = "
            SELECT id, title, color, description, created_at, icon 
            FROM tags";

        let mut stmt = self.conn.prepare(sql).map_err(|e| e.to_string())?;

        let tag_iter = stmt.query_map([], |row| {
            Ok(Tag {
                id: row.get(0)?,
                title: row.get(1)?,
                color: row.get(2)?,
                description: row.get(3).ok(),
                created_at: row.get(4)?,
                icon: row.get(5).ok(),
            })
        }).map_err(|e| e.to_string())?;

        let tags: Vec<Tag> = tag_iter.collect::<Result<_, _>>()
            .map_err(|e| e.to_string())?;

        Ok(tags)
    }
}

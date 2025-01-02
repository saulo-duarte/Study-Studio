use rusqlite::{params, Connection, Result};
use crate::db::book::Book;
use crate::db::tag::Tag;

pub struct TagRepository<'a> {
    conn: &'a mut Connection,
}

impl<'a> TagRepository<'a> {
    pub fn new(conn: &'a mut Connection) -> Self {
        Self { conn }
    }

    pub fn create_tag(&mut self, tag: &Tag) -> Result<i64> {
        let mut stmt = self.conn.prepare(
            "INSERT INTO tags (title, color, icon) VALUES (?, ?, ?)"
        )?;
        
        stmt.execute(params![tag.title, tag.color, tag.icon])?;
        
        Ok(self.conn.last_insert_rowid())
    }

    pub fn update_title(&mut self, id: i32, new_title: &str) -> Result<usize> {
        self.conn.execute(
            "UPDATE tags SET title = ? WHERE id = ?",
            params![new_title, id],
        )
    }

    pub fn update_color(&mut self, id: i32, new_color: &str) -> Result<usize> {
        self.conn.execute(
            "UPDATE tags SET color = ? WHERE id = ?",
            params![new_color, id],
        )
    }

    pub fn update_icon(&mut self, id: i32, new_icon: Option<&str>) -> Result<usize> {
        self.conn.execute(
            "UPDATE tags SET icon = ? WHERE id = ?",
            params![new_icon, id],
        )
    }

    pub fn delete_tag(&mut self, id: i32) -> Result<usize> {
        self.conn.execute("DELETE FROM tags WHERE id = ?", params![id])
    }

    pub fn get_all_tags(&self) -> Result<Vec<Tag>> {
        let mut stmt = self.conn.prepare("SELECT id, title, color, icon FROM tags")?;
        let tag_iter = stmt.query_map([], |row| {
            Ok(Tag {
                id: row.get(0)?,
                title: row.get(1)?,
                color: row.get(2)?,
                icon: row.get(3)?,
            })
        })?;

        let tags: Result<Vec<Tag>> = tag_iter.collect();
        tags
    }
}

use rusqlite::{params, Connection, Result};
use crate::db::models::book::Book;
use crate::db::models::tag::Tag;

pub struct BookRepository<'a> {
    conn: &'a mut Connection, 
}

impl<'a> BookRepository<'a> {
    pub fn new(conn: &'a mut Connection) -> Self {
        Self { conn }
    }

    pub fn create_book(&mut self, book: &Book) -> Result<i64> {
        let tx = self.conn.transaction()?;
        {
            let mut stmt = tx.prepare(
                "INSERT INTO books (title, author, file_path) 
                 VALUES(?, ?, ?)"
            )?;
            stmt.execute(params![
                book.title,
                book.author,
                book.file_path
            ])?;
        }
        let book_id = tx.last_insert_rowid();
        
        if let Some(tags) = &book.tags {
            for tag in tags {
                let mut stmt = tx.prepare(
                    "INSERT INTO book_tags (book_id, tag_id) 
                     VALUES(?, ?)"
                )?;
                stmt.execute(params![book_id, tag.id])?;
            }
        }
        
        tx.commit()?;
        Ok(book_id)
    }

    pub fn update_book(&mut self, book: &Book) -> Result<usize> {
        self.conn.execute(
            "UPDATE books SET title = ?, author = ?, file_path = ? WHERE id = ?",
            params![book.title, book.author, book.file_path, book.id],
        )
    }

    pub fn delete_book(&mut self, id: i64) -> Result<usize> {
        let tx = self.conn.transaction()?;
        let result = tx.execute(
            "DELETE FROM books WHERE id = ?",
            params![id]
        );
        tx.commit()?;
        result
    }

    pub fn get_book_by_id(&self, id: i64) -> Result<Option<Book>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, author, file_path FROM books WHERE id = ?"
        )?;
        
        let mut rows = stmt.query(params![id])?;
        
        if let Some(row) = rows.next()? {
            let book = Book {
                id: row.get(0)?,
                title: row.get(1)?,
                author: row.get(2)?,
                file_path: row.get(3)?,
                tags: Some(self.get_tags_by_book_id(row.get(0)?)?),
            };
            Ok(Some(book))
        } else {
            Ok(None)
        }
    }

    pub fn get_all_books(&self) -> Result<Vec<Book>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, author, file_path FROM books"
        )?;
        
        let book_iter = stmt.query_map([], |row| {
            Ok(Book {
                id: row.get(0)?,
                title: row.get(1)?,
                author: row.get(2)?,
                file_path: row.get(3)?,
                tags: Some(self.get_tags_by_book_id(row.get(0)?)?),
            })
        })?;
        
        book_iter.collect()
    }

    pub fn get_tags_by_book_id(&self, book_id: i64) -> Result<Vec<Tag>> {
        let mut stmt = self.conn.prepare(
            "SELECT t.id, t.title, t.color, t.icon 
             FROM tags t 
             JOIN book_tags bt ON bt.tag_id = t.id 
             WHERE bt.book_id = ?"
        )?;
        
        let tags = stmt.query_map(params![book_id], |row| {
            Ok(Tag {
                id: Some(row.get(0)?),
                title: row.get(1)?,
                color: row.get(2)?,
                icon: row.get(3)?,
            })
        })?;
        
        tags.collect()
    }

    pub fn add_tags_to_book(&mut self, book_id: i64, tags: Vec<Tag>) -> Result<()> {
        let tx = self.conn.transaction()?;
        for tag in tags {
            if let Some(tag_id) = tag.id {
                let mut stmt = tx.prepare(
                    "INSERT INTO book_tags (book_id, tag_id) 
                     VALUES(?, ?)"
                )?;
                stmt.execute(params![book_id, tag_id])?;
            }
        }
        tx.commit()?;
        Ok(())
    }

    pub fn remove_tags_from_book(&mut self, book_id: i64, tags: Vec<Tag>) -> Result<()> {
        let tx = self.conn.transaction()?;
        for tag in tags {
            if let Some(tag_id) = tag.id {
                let mut stmt = tx.prepare(
                    "DELETE FROM book_tags WHERE book_id = ? AND tag_id = ?"
                )?;
                stmt.execute(params![book_id, tag_id])?;
            }
        }
        tx.commit()?;
        Ok(())
    }
}
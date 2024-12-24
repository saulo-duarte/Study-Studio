use rusqlite::{Connection, Result, params};
use rusqlite::OptionalExtension;
use crate::db::models::task::Task;
use crate::db::models::tag::Tag;

pub struct TaskRepository {
    conn: Connection,
}

impl TaskRepository {
    pub fn new(conn: Connection) -> Self {
        TaskRepository { conn }
    }

    // Method to create a task and associate tags with it
    pub fn create(&mut self, task: &Task) -> Result<(), String> {
        // Begin a transaction to ensure atomicity
        let tx = self.conn.transaction().map_err(|e| e.to_string())?;
    
        // Insert the task into the `tasks` table
        let query = "
            INSERT INTO tasks (
                title, 
                description, 
                status, 
                created_at, 
                due_date, 
                completed_at, 
                document_id
            ) 
            VALUES (?, ?, ?, ?, ?, ?, ?)
            RETURNING id";
        
        // Use `params!` to pass the values as parameters
        let task_id: i32 = tx
            .query_row(query, params![
                task.title,
                task.description,
                task.status,
                task.created_at,
                task.due_date,
                task.completed_at,
                task.document_id
            ], |row| row.get(0))
            .map_err(|e| e.to_string())?;
    
        // Associate the tags with the task
        for tag in &task.tags {
            let tag_query = "
                INSERT INTO task_tags (task_id, tag_id) 
                VALUES (?, ?)";
            
            tx.execute(tag_query, params![task_id, tag.id])
                .map_err(|e| e.to_string())?;
        }
    
        // Commit the transaction
        tx.commit().map_err(|e| e.to_string())?;
    
        Ok(())
    }
    

    // Method to retrieve a task by its ID and load associated tags
    pub fn get_by_id(&self, id: i32) -> Result<Option<Task>, String> {
        let query = "
            SELECT id, title, description, status, created_at, due_date, completed_at, document_id
            FROM tasks
            WHERE id = ?
        ";

        let task = self.conn
            .query_row(query, params![id], |row| {
                Ok(Task {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    description: row.get(2).ok(),
                    status: row.get(3)?,
                    created_at: row.get(4)?,
                    due_date: row.get(5).ok(),
                    completed_at: row.get(6).ok(),
                    document_id: row.get(7).ok(),
                    tags: Vec::new(), // We will load the tags later
                })
            })
            .optional()
            .map_err(|e| e.to_string())?;

        match task {
            Some(mut task) => {
                // Now, load the tags associated with the task
                let tag_query = "
                    SELECT t.id, t.title, t.color, t.description, t.created_at, t.icon
                    FROM tags t
                    JOIN task_tags tt ON tt.tag_id = t.id
                    WHERE tt.task_id = ?
                ";

                let mut stmt = self.conn.prepare(tag_query).map_err(|e| e.to_string())?;
                let tag_iter = stmt.query_map(params![id], |row| {
                    Ok(Tag {
                        id: row.get(0)?,
                        title: row.get(1)?,
                        color: row.get(2)?,
                        description: row.get(3).ok(),
                        created_at: row.get(4)?,
                        icon: row.get(5).ok(),
                    })
                }).map_err(|e| e.to_string())?;

                for tag in tag_iter {
                    task.tags.push(tag.map_err(|e| e.to_string())?);
                }

                Ok(Some(task))
            },
            None => Ok(None),
        }
    }

    // Method to update a task and its associated tags
    pub fn update(&mut self, task: &Task) -> Result<(), String> {
        // Begin a transaction to ensure atomicity
        let tx = self.conn.transaction().map_err(|e| e.to_string())?;
    
        // Update the task in the `tasks` table
        let query = "
            UPDATE tasks 
            SET title = ?, description = ?, status = ?, created_at = ?, due_date = ?, completed_at = ?, document_id = ?
            WHERE id = ?
        ";
        
        tx.execute(query, params![
            task.title,
            task.description,
            task.status,
            task.created_at,
            task.due_date,
            task.completed_at,
            task.document_id,
            task.id
        ])
        .map_err(|e| e.to_string())?;
    
        // Delete existing associations with tags
        let delete_tags_query = "DELETE FROM task_tags WHERE task_id = ?";
        tx.execute(delete_tags_query, params![task.id])
            .map_err(|e| e.to_string())?;
    
        // Reassociate the new tags with the task
        for tag in &task.tags {
            let tag_query = "
                INSERT INTO task_tags (task_id, tag_id) 
                VALUES (?, ?)";
            
            tx.execute(tag_query, params![task.id, tag.id])
                .map_err(|e| e.to_string())?;
        }
    
        // Commit the transaction
        tx.commit().map_err(|e| e.to_string())?;
        Ok(())
    }

    // Method to delete a task and its associated tags
pub fn delete(&mut self, id: i32) -> Result<(), String> {
    // Begin a transaction to ensure atomicity
    let tx = self.conn.transaction().map_err(|e| e.to_string())?;

    // Delete associations with tags
    let delete_tags_query = "DELETE FROM task_tags WHERE task_id = ?";
    tx.execute(delete_tags_query, params![id])
        .map_err(|e| e.to_string())?;

    // Delete the task itself
    let query = "DELETE FROM tasks WHERE id = ?";
    tx.execute(query, params![id])
        .map_err(|e| e.to_string())?;

    // Commit the transaction
    tx.commit().map_err(|e| e.to_string())?;
    Ok(())
    
    }

    // Method to get all tasks along with their tags
    pub fn get_all(&self) -> Result<Vec<Task>, String> {
        let query = "
            SELECT id, title, description, status, created_at, due_date, completed_at, document_id
            FROM tasks
        ";

        let mut stmt = self.conn.prepare(query).map_err(|e| e.to_string())?;
        let task_iter = stmt.query_map([], |row| {
            Ok(Task {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2).ok(),
                status: row.get(3)?,
                created_at: row.get(4)?,
                due_date: row.get(5).ok(),
                completed_at: row.get(6).ok(),
                document_id: row.get(7).ok(),
                tags: Vec::new(), // We will load the tags later
            })
        }).map_err(|e| e.to_string())?;

        let mut tasks = Vec::new();
        for task in task_iter {
            let mut task = task.map_err(|e| e.to_string())?;

            // Load the associated tags for each task
            let tag_query = "
                SELECT t.id, t.title, t.color, t.description, t.created_at, t.icon
                FROM tags t
                JOIN task_tags tt ON tt.tag_id = t.id
                WHERE tt.task_id = ?
            ";

            let mut stmt = self.conn.prepare(tag_query).map_err(|e| e.to_string())?;
            let tag_iter = stmt.query_map(params![task.id], |row| {
                Ok(Tag {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    color: row.get(2)?,
                    description: row.get(3).ok(),
                    created_at: row.get(4)?,
                    icon: row.get(5).ok(),
                })
            }).map_err(|e| e.to_string())?;

            for tag in tag_iter {
                task.tags.push(tag.map_err(|e| e.to_string())?);
            }

            tasks.push(task);
        }

        Ok(tasks)
    }
}

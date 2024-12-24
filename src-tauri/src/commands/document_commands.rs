use crate::db::repositories::document_repository::DocumentRepository;
use crate::db::models::document::Document;
use rusqlite::Connection;
use chrono::Utc;

pub struct DocumentCommands<'a> {
    repo: DocumentRepository<'a>,
}

impl<'a> DocumentCommands<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        let repo = DocumentRepository::new(conn);
        Self { repo }
    }

    pub fn create_document(&self, document: Document) -> Result<(), String> {
        self.repo.create(&document)
    }

    pub fn get_document_by_id(&self, id: i32) -> Result<Option<Document>, String> {
        self.repo.get_by_id(id)
    }

    pub fn update_document(&self, document: Document) -> Result<(), String> {
        self.repo.update(&document)
    }

    pub fn delete_document(&self, id: i32) -> Result<(), String> {
        self.repo.delete(id)
    }

    pub fn list_all_documents(&self) -> Result<Vec<Document>, String> {
        self.repo.get_all()
    }

    pub fn search_documents_by_title(&self, title: &str) -> Result<Vec<Document>, String> {
        let all_documents = self.repo.get_all()?;
        let filtered_documents: Vec<Document> = all_documents
            .into_iter()
            .filter(|doc| doc.title.to_lowercase().contains(&title.to_lowercase()))
            .collect();
        Ok(filtered_documents)
    }

    pub fn mark_document_as_accessed(&self, id: i32) -> Result<(), String> {
        if let Some(mut document) = self.repo.get_by_id(id)? {
            document.last_accessed = Some(Utc::now()); 
            self.repo.update(&document)
        } else {
            Err("Document not found".to_string())
        }
    }
    
}

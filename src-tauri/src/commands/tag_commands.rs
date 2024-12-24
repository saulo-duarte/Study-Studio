
use crate::db::repositories::tag_repository::TagRepository;
use crate::db::models::tag::Tag;
use rusqlite::Connection;

pub struct TagCommands<'a> {
    repo: TagRepository<'a>,
}

impl<'a> TagCommands<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        let repo = TagRepository::new(conn);
        Self { repo }
    }

    pub fn create_tag(&self, tag: Tag) -> Result<(), String> {
        self.repo.create(&tag)
    }

    pub fn get_tag_by_id(&self, id: i64) -> Result<Option<Tag>, String> {
        self.repo.get_by_id(id)
    }

    pub fn update_tag(&self, tag: Tag) -> Result<(), String> {
        self.repo.update(&tag)
    }

    pub fn delete_tag(&self, id: i64) -> Result<(), String> {
        self.repo.delete(id)
    }

    pub fn list_all_tags(&self) -> Result<Vec<Tag>, String> {
        self.repo.get_all()
    }

    pub fn search_tags_by_title(&self, title: &str) -> Result<Vec<Tag>, String> {
        let all_tags = self.repo.get_all()?;
        let filtered_tags: Vec<Tag> = all_tags
            .into_iter()
            .filter(|tag| tag.title.to_lowercase().contains(&title.to_lowercase()))
            .collect();
        Ok(filtered_tags)
    }

    pub fn count_tags(&self) -> Result<usize, String> {
        let all_tags = self.repo.get_all()?;
        Ok(all_tags.len())
    }

    pub fn tag_summary(&self) -> Result<Vec<(String, String)>, String> {
        let all_tags = self.repo.get_all()?;
        let summaries: Vec<(String, String)> = all_tags
            .iter()
            .map(|tag| (tag.title.clone(), tag.color.clone()))
            .collect();
        Ok(summaries)
    }
}

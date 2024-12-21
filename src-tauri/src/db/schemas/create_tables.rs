/// SQL statement to create the `user` table.
pub const CREATE_USER_TABLE: &str = "
    CREATE TABLE IF NOT EXISTS user (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        email TEXT UNIQUE NOT NULL,
        created_at TEXT NOT NULL,
        last_login TEXT NOT NULL,
        interesting_fields TEXT,
        available_days TEXT,
        status TEXT NOT NULL DEFAULT 'active' 
    );
";


/// SQL statement to create the `document` table.
pub const CREATE_DOCUMENT_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS document (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    original_filename TEXT NOT NULL,
    stored_filename TEXT NOT NULL,
    file_path TEXT NOT NULL,
    file_size INTEGER NOT NULL,
    mime_type TEXT NOT NULL,
    hash TEXT NOT NULL,
    page_count INTEGER NOT NULL,
    created_at TEXT NOT NULL,
    last_accessed TEXT,
    thumbnail_path TEXT,
    tags TEXT
);
"#;

/// SQL statement to create the `task` table.
pub const CREATE_TASK_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS task (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    due_date TEXT,
    completed_at TEXT,
    document_id INTEGER,
    tags TEXT,
    FOREIGN KEY (document_id) REFERENCES document (id)
);
"#;

/// SQL statement to create the `tag` table.
pub const CREATE_TAG_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS tag (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    color TEXT NOT NULL,
    description TEXT,
    created_at TEXT NOT NULL,
    icon TEXT
);
"#;

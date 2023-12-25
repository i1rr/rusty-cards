use rusqlite::{Connection, params, Result};


pub struct FlashCard {
    id: u32,
    priority: u8,
    word: String,
    translate: String,
    example: Option<String>,
    definition: Option<String>,
}

impl FlashCard {

    pub fn empty() -> FlashCard {
        FlashCard {
            id: 0,
            priority: 0,
            word: String::new(),
            translate: String::new(),
            example: None,
            definition: None
        }
    }
    fn delete_by_id(word_entry: FlashCard) -> Result<usize> {
        let conn = Connection::open("dict.db")?;
        let rows_affected = conn.execute(
            "DELETE FROM dictionary WHERE id = ?1",
            params![word_entry.id],
        )?;
        Ok(rows_affected)
    }

    pub fn fetch_next_ten() -> Result<Vec<FlashCard>> {
        let conn = Connection::open("dict.db")?;
        let mut stmt = conn.prepare(
            "SELECT id, priority, word, translate, example, definition FROM dictionary
         ORDER BY priority DESC LIMIT 10",
        )?;

        let entries_iter = stmt.query_map([], |row| FlashCard::from_row(row))?;

        let mut entries = Vec::new();
        for entry in entries_iter {
            entries.push(entry?);
        }

        Ok(entries)
    }

    fn add(word_entry: FlashCard) -> Result<usize> {

        let conn = Connection::open("dict.db")?;
        let rows_affected = conn.execute(
            "INSERT INTO dictionary (word, translate, example, definition) VALUES (?1, ?2, ?3, ?4)",
            params![
            word_entry.word,
            word_entry.translate,
            word_entry.example,
            word_entry.definition
        ],
        )?;
        Ok(rows_affected)
    }

    fn edit(word_entry: FlashCard) -> Result<usize> {

        let conn = Connection::open("dict.db")?;
        let rows_affected = conn.execute(
            "UPDATE dictionary SET word = ?1, translate = ?2, example = ?3, definition = ?4 WHERE id = ?5",
            params![
            word_entry.word,
            word_entry.translate,
            word_entry.example,
            word_entry.definition,
            word_entry.id],
        )?;
        Ok(rows_affected)
    }

    fn decrease_priority(word_entry: FlashCard) -> Result<usize> {

        let conn = Connection::open("dict.db")?;
        let rows_affected = conn.execute(
            "UPDATE dictionary SET priority = priority - 1 WHERE id = ?1",
            params![word_entry.id],
        )?;
        Ok(rows_affected)
    }

    fn increase_priority(word_entry: FlashCard) -> Result<usize> {

        let conn = Connection::open("dict.db")?;
        let rows_affected = conn.execute(
            "UPDATE dictionary SET priority = priority + 1 WHERE id = ?1",
            params![word_entry.id],
        )?;
        Ok(rows_affected)
    }

    pub fn initialize_db() -> Result<usize> {

        let conn = Connection::open("dict.db")?;
        let rows_affected = conn.execute(
            "CREATE TABLE IF NOT EXISTS dictionary (
            id INTEGER PRIMARY KEY,
            priority INTEGER NOT NULL DEFAULT 10,
            word TEXT NOT NULL,
            translate TEXT NOT NULL,
            example TEXT,
            definition TEXT
        )",
            [],
        )?;

        FlashCard::meta_table()?;

        Ok(rows_affected)
    }

    fn meta_table() -> Result<usize> {
        let conn = Connection::open("dict.db")?;
        let rows_affected = conn.execute(
            "CREATE TABLE IF NOT EXISTS meta (
            id INTEGER PRIMARY KEY,
            name TEXT,
            cards_shown_total INTEGER NOT NULL DEFAULT 0,
            counter_resettable INTEGER NOT NULL DEFAULT 0,
            words_mastered INTEGER NOT NULL DEFAULT 0,
            words_in_db INTEGER NOT NULL DEFAULT 0,
            hardest_word TEXT
        )",
            [],
        )?;
        Ok(rows_affected)
    }

    fn from_row(row: &rusqlite::Row) -> Result<FlashCard> {
        Ok(FlashCard {
            id: row.get(0)?,
            priority: row.get(1)?,
            word: row.get(2)?,
            translate: row.get(3)?,
            example: row.get(4)?,
            definition: row.get(5)?,
        })
    }
}

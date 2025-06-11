use serde::{Serialize, Deserialize};
use std::{fs, io::{self, Write}};

const NOTES_FILE: &str = "notes.json";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Note {
        pub id: u32,
        pub title: String,
        pub content: String,
}

pub struct NoteKeeper {
        notes: Vec<Note>,
        next_id: u32,
}

impl NoteKeeper {
    pub fn new() -> Self {
        
        let allnotes: Vec<Note> = match fs::read_to_string(NOTES_FILE) {
            Ok(data) => serde_json::from_str(&data).unwrap_or_else(|_| vec![]),
            Err(_) => vec![],
        };

        let next_id = allnotes.iter().map(|n| n.id).max().unwrap_or(0) + 1;
        Self{notes:allnotes, next_id:next_id }
    }

    fn save_notes(&self){
       if let Ok(json) = serde_json::to_string_pretty(&self.notes){
            let _ = fs::write(NOTES_FILE, json);
        }
    }

    pub fn add_note(&mut self, title: &str, content: &str) {
        let note = Note {
                       id: self.next_id,
                       title: title.to_string(),
                       content: content.to_string(),
                   };
         self.notes.push(note);
         self.next_id += 1;
         self.save_notes();
         println!("Note added.");
    } 

    pub fn list_notes(&self) {
        for note in &self.notes {
            println!("[{}] {}", note.id, note.title);
        }
    }

     pub fn view_note(&self, id: u32) -> Option<&Note> {
         self.notes.iter().find(|note| note.id == id)
     }

     pub fn delete_note(&mut self, id: u32) {
         let original = self.notes.len();
         self.notes.retain(|note| note.id != id);
         if self.notes.len() < original {
                self.save_notes();
               println!("Deleted note {}", id);
         } else {
               eprintln!("Note with ID {} not found.", id);
         }
      }
}


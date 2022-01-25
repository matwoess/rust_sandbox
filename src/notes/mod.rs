use crate::notes::note::{DIVIDER, Note, PAD_TEXT, PAD_TITLE};
use crate::notes::NoteAction::{Delete, Edit, Exit, Invalid, List, New};
use crate::util;
use crate::util::prompt_for_sting;

mod note;

enum NoteAction {
    List,
    New,
    Delete(usize),
    Edit(usize),
    Exit,
    Invalid,
}

impl NoteAction {
    fn all_actions() -> [&'static str; 5] {
        ["list", "new", "delete", "edit", "exit"]
    }
}

pub fn main() {
    println!("> starting \"notes\" module");
    let mut my_notes: Vec<Note> = Vec::new();
    let n = Note::new(
        String::from("Shopping list"),
        String::from("Buy: eggs, milk, bread"),
    );
    my_notes.push(n);
    loop {
        let action = query_action(&my_notes);
        match action {
            List => list_notes(&my_notes),
            New => add_note(&mut my_notes),
            Delete(idx) => delete_note(&mut my_notes, idx),
            Edit(idx) => edit_note(&mut my_notes, idx),
            Exit => break,
            Invalid => continue,
        }
    }
    println!("exiting...")
}

fn query_action(notes: &Vec<Note>) -> NoteAction {
    println!("\nAvailable actions:");
    for (i, val) in NoteAction::all_actions().iter().enumerate() {
        println!("{}) {}", i, val);
    }
    print!("What do you want to do?: ");
    let chosen = util::prompt_for_usize();
    let action = match chosen {
        0 => List,
        1 => New,
        2 => {
            let idx = query_list_index(&notes);
            if idx == usize::MAX { Invalid } else { Delete(idx) }
        }
        3 => {
            let idx = query_list_index(&notes);
            if idx == usize::MAX { Invalid } else { Edit(idx) }
        }
        4 => Exit,
        _ => {
            println!("Invalid option!");
            Invalid
        }
    };
    action
}

fn query_list_index(list: &&Vec<Note>) -> usize {
    if list.len() == 0 {
        println!("List of notes is empty!");
        return usize::MAX;
    }
    print!("Element at which index ({}..{})?: ", 0, list.len());
    util::prompt_for_usize()
}

fn edit_note(notes: &mut Vec<Note>, idx: usize) {
    if idx >= notes.len() {
        println!("Invalid index!");
        return;
    }
    let note : &mut Note = notes.get_mut(idx).expect("Error getting note!");
    println!("Old note title     : {}", note.title);
    print!("New (empty => keep): ");
    let title = prompt_for_sting();
    println!("Old note text      : {}", note.text);
    print!("New (empty => keep): ");
    let text = prompt_for_sting();
    note.update(title, text);
}

fn delete_note(notes: &mut Vec<Note>, idx: usize) {
    if idx >= notes.len() {
        println!("Invalid index!");
        return;
    }
    notes.remove(idx);
}

fn list_notes(notes: &Vec<Note>) {
    if notes.len() == 0 {
        println!("List of notes is empty!");
        return;
    }
    for n in notes {
        println!("{}", n);
    }
}

fn add_note(notes: &mut Vec<Note>) {
    println!("{}", DIVIDER);
    print!("{}", PAD_TITLE);
    let title = prompt_for_sting();
    print!("{}", PAD_TEXT);
    let text = prompt_for_sting();
    println!("{}", DIVIDER);
    let new_note = Note::new(title, text);
    notes.push(new_note);
}

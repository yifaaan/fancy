use display_utils::display_entries;
use file_entry::FileEntry;
use std::{env, fs};
use terminal_size::{terminal_size, Height, Width};

mod display_utils;
mod emoji_utils;
mod file_entry;
mod terminal_size;

fn main() {
    // println!("{:?}", terminal_size());
    let current_dir = env::current_dir().unwrap();
    // 粗体
    println!("\x1B[1m{}\x1B[0m", current_dir.display());

    let mut entries: Vec<FileEntry> = fs::read_dir(&current_dir)
        .unwrap()
        .filter_map(Result::ok)
        .map(|entry| FileEntry::new(entry.path()))
        .collect();

    entries.sort();

    let (Width(term_width), _) = terminal_size().unwrap_or((Width(80), Height(24)));
    display_entries(&entries, term_width as usize);
}

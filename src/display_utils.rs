use unicode_width::UnicodeWidthStr;

use crate::file_entry::FileEntry;

pub fn display_entries(entries: &[FileEntry], term_width: usize) {
    let max_columns = 4;

    let max_width = entries
        .iter()
        .map(|entry| entry.emoji.width() + 1 + UnicodeWidthStr::width(entry.name.as_str()))
        .max()
        .unwrap_or(0);

    let column_width = max_width + 2;

    let num_columns = (term_width / column_width).min(max_columns).max(1);
    let num_entries = entries.len();
    let num_rows = (num_entries + num_columns - 1) / num_columns;

    for row in 0..num_rows {
        for col in 0..num_columns {
            let idx = row + col * num_rows;
            if idx < num_entries {
                let entry = &entries[idx];
                let display_width =
                    entry.emoji.width() + 1 + UnicodeWidthStr::width(entry.name.as_str());
                print!("{} {}", entry.emoji, entry.name);
                if col < num_columns - 1 {
                    print!("{:width$}", "", width = column_width - display_width);
                }
            }
        }
        println!()
    }
}

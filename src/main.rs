use std::{env, fs, os::unix::fs::PermissionsExt, path::Path};
use terminal_size::{terminal_size, Height, Width};

mod terminal_size;

fn get_emoji(path: &Path) -> &'static str {
    if path.is_symlink() {
        return if path.is_dir() { "🔗📁" } else { "🔗" };
    }
    if path.is_dir() {
        return "📁";
    }
    let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

    match extension {
        "jpg" | "jpeg" | "png" | "gif" | "bmp" | "svg" | "webp" => "🎨",
        "mp4" | "avi" | "mkv" | "mov" | "flv" | "wmv" | "webm" => "🎬",
        "mp3" | "wav" | "ogg" | "flac" | "m4a" | "aac" => "🎧",
        "zip" | "tar" | "gz" | "bz2" | "xz" | "7z" | "rar" => "📦",
        "py" | "js" | "html" | "css" | "cpp" | "c" | "java" | "go" | "rb" | "rs" | "php" => "👨‍💻",
        "txt" | "md" | "rst" | "log" => "📝",
        "ttf" | "otf" | "woff" | "woff2" => "🔤",
        "pdf" => "📚",
        "xls" | "xlsx" | "csv" => "📊",
        "ppt" | "pptx" => "📽️",
        "conf" | "config" | "cfg" | "ini" | "yaml" | "yml" | "json" | "xml" => "⚙️",
        _ => {
            if file_name.starts_with('.') {
                "⚙️"
            } else if is_executable(path) {
                "🚀"
            } else {
                "📰"
            }
        }
    }
}

fn is_executable(path: &Path) -> bool {
    fs::metadata(path)
        .map(|m| m.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

fn get_order(path: &Path) -> u8 {
    if path.is_symlink() {
        if path.is_dir() {
            0
        } else {
            1
        }
    } else if path.is_dir() {
        0
    } else if is_executable(path) {
        2
    } else if path
        .file_name()
        .and_then(|n| n.to_str())
        .map_or(false, |n| n.starts_with('.'))
    {
        3
    } else {
        4
    }
}
fn main() {
    // println!("{:?}", terminal_size());
    let current_dir = env::current_dir().unwrap();
    // 粗体
    println!("\x1B[1m{}\x1B[0m", current_dir.display());

    let mut entries: Vec<_> = fs::read_dir(&current_dir)
        .unwrap()
        .filter_map(Result::ok)
        .map(|entry| {
            let path = entry.path();
            // println!("{}", path.display());
            let file_name = path.file_name().unwrap().to_string_lossy().into_owned();
            let emoji = get_emoji(&path);
            (emoji, file_name, path)
        })
        .collect();

    entries.sort_by_key(|(_, name, path)| (get_order(path), name.clone()));

    let (Width(term_width), _) = terminal_size().unwrap_or((Width(80), Height(24)));
    let term_width = term_width as usize;
    let max_name_width = entries
        .iter()
        .map(|(_, name, _)| name.len())
        .max()
        .unwrap_or(0);
    let column_width = max_name_width + 4;
    // 1..=4
    let num_columns = (term_width / column_width).min(4).max(1);

    for chunk in entries.chunks(num_columns) {
        for (emoji, name, _) in chunk {
            // 宽度由变量width设置
            print!("{} {:<width$}", emoji, name, width = max_name_width + 1);
        }
        println!();
    }
}

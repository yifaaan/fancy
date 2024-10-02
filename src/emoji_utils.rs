use std::{fs, os::unix::fs::PermissionsExt, path::Path};

pub fn get_emoji(path: &Path) -> &'static str {
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

pub fn is_executable(path: &Path) -> bool {
    fs::metadata(path)
        .map(|m| m.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

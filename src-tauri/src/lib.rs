use serde::Serialize;
use std::{
    fs,
    path::{Path, PathBuf},
};
use tauri::{Emitter, Manager};

const MARKDOWN_EXTENSIONS: [&str; 3] = ["md", "mdx", "markdown"];

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct MarkdownDocument {
    path: String,
    content: String,
    siblings: Vec<String>,
}

fn is_markdown(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .map(|extension| {
            MARKDOWN_EXTENSIONS
                .iter()
                .any(|allowed| extension.eq_ignore_ascii_case(allowed))
        })
        .unwrap_or(false)
}

fn normalize_argument(argument: &str, working_directory: &Path) -> Option<PathBuf> {
    let value = argument.trim().trim_matches(['"', '\'']);
    if value.is_empty() || value.starts_with('-') {
        return None;
    }

    let path = PathBuf::from(value);
    let absolute = if path.is_absolute() {
        path
    } else {
        working_directory.join(path)
    };

    is_markdown(&absolute).then_some(absolute)
}

fn find_markdown_argument(arguments: &[String], working_directory: &Path) -> Option<PathBuf> {
    arguments
        .iter()
        .skip(1)
        .find_map(|argument| normalize_argument(argument, working_directory))
}

#[tauri::command]
fn startup_file() -> Option<String> {
    let arguments: Vec<String> = std::env::args().collect();
    let working_directory = std::env::current_dir().ok()?;
    find_markdown_argument(&arguments, &working_directory)
        .map(|path| path.to_string_lossy().into_owned())
}

#[tauri::command]
fn open_markdown(path: String) -> Result<MarkdownDocument, String> {
    let requested_path = PathBuf::from(path);
    if !is_markdown(&requested_path) {
        return Err("该文件不是受支持的 Markdown 格式".into());
    }

    let canonical_path = requested_path
        .canonicalize()
        .map_err(|error| format!("找不到文件：{error}"))?;
    let content =
        fs::read_to_string(&canonical_path).map_err(|error| format!("读取文件失败：{error}"))?;

    let mut siblings = canonical_path
        .parent()
        .and_then(|directory| fs::read_dir(directory).ok())
        .into_iter()
        .flatten()
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|entry_path| entry_path.is_file() && is_markdown(entry_path))
        .map(|entry_path| entry_path.to_string_lossy().into_owned())
        .collect::<Vec<_>>();

    siblings.sort_by_key(|entry| entry.to_lowercase());

    Ok(MarkdownDocument {
        path: canonical_path.to_string_lossy().into_owned(),
        content,
        siblings,
    })
}

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(target_os = "windows")]
            if let Some(window) = app.get_webview_window("main") {
                let _ = window_vibrancy::apply_acrylic(&window, Some((18, 34, 48, 126)));
            }
            Ok(())
        })
        .plugin(tauri_plugin_single_instance::init(|app, arguments, cwd| {
            let working_directory = PathBuf::from(cwd);
            if let Some(path) = find_markdown_argument(&arguments, &working_directory) {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.unminimize();
                    let _ = window.set_focus();
                    let _ = window.emit("open-file-from-args", path.to_string_lossy().into_owned());
                }
            }
        }))
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![startup_file, open_markdown])
        .run(tauri::generate_context!())
        .expect("error while running MarkGlass");
}

// =============================================
// Designs: шрифты и шаблоны дизайнов
// =============================================

use crate::app_state::App;
use std::path::{Path, PathBuf};

// =============================================
// Встроенные шрифты
// =============================================
pub fn available_fonts() -> Vec<String> {
    vec![
        "Helvetica Neue".to_string(),
        "Noto Sans".to_string(),
        "Noto Serif".to_string(),
        "Open Sans".to_string(),
        "Roboto".to_string(),
    ]
}

// =============================================
// Общий обход .typ-файлов
// Возвращает пары (полный путь, относительный путь с '/').
// =============================================
pub fn collect_typ_files(root: &Path) -> Vec<(PathBuf, String)> {
    let mut files = Vec::new();
    let mut stack = vec![root.to_path_buf()];

    while let Some(dir) = stack.pop() {
        let entries = match std::fs::read_dir(&dir) {
            Ok(entries) => entries,
            Err(_) => continue,
        };

        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }

            let is_typ = path
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext.eq_ignore_ascii_case("typ"))
                .unwrap_or(false);
            if !is_typ {
                continue;
            }

            if let Ok(relative) = path.strip_prefix(root) {
                let normalized = relative.to_string_lossy().replace('\\', "/");
                files.push((path, normalized));
            }
        }
    }

    files.sort_by(|a, b| a.1.cmp(&b.1));
    files
}

// =============================================
// Список дизайнов для UI
// =============================================
pub fn discover_designs() -> Vec<String> {
    let root = Path::new("templates").join("designs");
    let mut designs = collect_typ_files(&root)
        .into_iter()
        .map(|(_, relative)| relative)
        .filter(|name| name != "current.typ")
        .collect::<Vec<_>>();

    designs.sort();
    designs
}

// =============================================
// Выбор текущего значения с fallback
// =============================================
pub fn select_existing_or_fallback(current: &str, options: &[String], fallback: &str) -> String {
    if options.iter().any(|opt| opt == current) {
        return current.to_string();
    }
    if options.iter().any(|opt| opt == fallback) {
        return fallback.to_string();
    }
    options.first().cloned().unwrap_or_default()
}

// =============================================
// Обновление списка дизайнов в состоянии App
// =============================================
pub fn refresh_designs(app: &mut App) {
    let previous_design = app.selected_design.clone();
    app.available_designs = discover_designs();
    app.selected_design =
        select_existing_or_fallback(&previous_design, &app.available_designs, "default.typ");

    if app.available_designs.is_empty() {
        app.status_message = "В папке templates/designs не найдено .typ дизайнов".to_string();
    } else {
        app.status_message = "Список дизайнов обновлён".to_string();
    }
}

// =============================================
// Корень папки шаблонов
// =============================================
pub fn templates_root() -> PathBuf {
    Path::new("templates").to_path_buf()
}

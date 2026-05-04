// =============================================
// Renderer: Typst -> SVG/PDF
// =============================================

use crate::app_state::{App, Message, TypstInputs};
use crate::designs;
use iced::Task;
use std::path::Path;

// =============================================
// Добавляем все runtime .typ-файлы из templates
// =============================================
fn add_runtime_templates(
    mut document: typst_bake::Document,
) -> Result<typst_bake::Document, String> {
    let root = designs::templates_root();
    if !root.exists() {
        return Ok(document);
    }

    for (path, virtual_path) in designs::collect_typ_files(&root) {
        let bytes = std::fs::read(&path)
            .map_err(|e| format!("Не удалось прочитать шаблон {}: {e}", path.display()))?;

        document = document
            .add_file(&virtual_path, bytes)
            .map_err(|e| format!("Не удалось добавить шаблон {}: {e}", virtual_path))?;
    }

    Ok(document)
}

// =============================================
// Сборка документа для выбранного дизайна
// =============================================
pub fn build_document(
    design_name: String,
    inputs: TypstInputs,
) -> Result<typst_bake::Document, String> {
    let mut resolved_inputs = inputs;
    let mut document = typst_bake::document!("blanks.typ");
    document = add_runtime_templates(document)?;

    let selected = design_name.trim();
    if selected.is_empty() {
        return Err("Не выбран дизайн".to_string());
    }

    let selected_path = Path::new("templates").join("designs").join(selected);
    let selected_bytes = std::fs::read(&selected_path).map_err(|e| {
        format!(
            "Не удалось прочитать выбранный дизайн {}: {e}",
            selected_path.display()
        )
    })?;

    document = document
        .add_file("designs/current.typ", selected_bytes)
        .map_err(|e| format!("Не удалось подменить активный дизайн: {e}"))?;

    if !resolved_inputs.logo_path.trim().is_empty() {
        let host_path = resolved_inputs.logo_path.clone();
        let ext = Path::new(&host_path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("png");
        let virtual_path = format!("runtime/logo.{}", ext);

        let bytes =
            std::fs::read(&host_path).map_err(|e| format!("Не удалось прочитать логотип: {e}"))?;

        document = document
            .add_file(&virtual_path, bytes)
            .map_err(|e| format!("Не удалось добавить логотип в шаблон: {e}"))?;

        resolved_inputs.logo_path = virtual_path;
    }

    Ok(document.with_inputs(resolved_inputs))
}

// =============================================
// Async рендер SVG
// =============================================
pub fn render_task(design_name: String, inputs: TypstInputs) -> Task<Message> {
    Task::perform(
        async move {
            match build_document(design_name, inputs)
                .and_then(|doc| doc.to_svg().map_err(|e| e.to_string()))
            {
                Ok(svgs) => Message::SvgLoaded(Ok(svgs)),
                Err(e) => Message::SvgLoaded(Err(e)),
            }
        },
        |msg| msg,
    )
}

// =============================================
// Async рендер текущего состояния
// =============================================
pub fn render_current(app: &App) -> Task<Message> {
    if app.selected_design.is_empty() {
        Task::none()
    } else {
        render_task(app.selected_design.clone(), app.to_inputs())
    }
}

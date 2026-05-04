// =============================================
// Main: точка входа и update-цикл
// =============================================

mod app_state;
mod designs;
mod renderer;
mod ui;

use app_state::{App, Message, SheetsMode};
use iced::Task;
use iced::Theme;
use renderer::render_current;

// =============================================
// Boot: начальное состояние + первый рендер
// =============================================
fn boot() -> (App, Task<Message>) {
    let mut app = App {
        start_team: 1,
        finish_team: 20,
        cols: 4,
        rows: 9,
        landscape: false,
        location: String::new(),
        team_prefix: "Команда №".to_string(),
        font_name: "Helvetica Neue".to_string(),
        available_fonts: designs::available_fonts(),
        selected_design: "default.typ".to_string(),
        available_designs: Vec::new(),
        sheets_mode: SheetsMode::One,
        logo_path: String::new(),
        team_names: Vec::new(),
        teams_file_path: String::new(),
        dark_theme: true,
        status_message: "Готово".to_string(),
        svg_pages: Vec::new(),
    };

    app.font_name = designs::select_existing_or_fallback(
        &app.font_name,
        &app.available_fonts,
        "Helvetica Neue",
    );
    designs::refresh_designs(&mut app);

    let task = render_current(&app);
    (app, task)
}

// =============================================
// Update: обработка сообщений и async-результатов
// =============================================
fn update(app: &mut App, msg: Message) -> Task<Message> {
    match msg {
        Message::StartTeamChanged(v) => {
            app.start_team = v;
            if app.start_team > app.finish_team {
                app.finish_team = app.start_team;
            }
            return render_current(app);
        }
        Message::FinishTeamChanged(v) => {
            app.finish_team = v;
            if app.finish_team < app.start_team {
                app.start_team = app.finish_team;
            }
            return render_current(app);
        }
        Message::ColsChanged(v) => {
            app.cols = v;
            return render_current(app);
        }
        Message::RowsChanged(v) => {
            app.rows = if app.sheets_mode == SheetsMode::Two {
                v.max(2)
            } else {
                v
            };
            return render_current(app);
        }
        Message::LandscapeChanged(v) => {
            app.landscape = v;
            return render_current(app);
        }
        Message::LocationChanged(v) => {
            app.location = v;
            return render_current(app);
        }
        Message::TeamPrefixChanged(v) => {
            app.team_prefix = v;
            return render_current(app);
        }
        Message::FontChanged(v) => {
            app.font_name = v;
            return render_current(app);
        }
        Message::DesignChanged(v) => {
            app.selected_design = v;
            return render_current(app);
        }
        Message::TwoSheetsChanged(enabled) => {
            app.sheets_mode = if enabled {
                SheetsMode::Two
            } else {
                SheetsMode::One
            };
            if app.sheets_mode == SheetsMode::Two {
                app.rows = app.rows.max(2);
            }
            return render_current(app);
        }
        Message::LogoPathChanged(v) => {
            app.logo_path = v;
            return render_current(app);
        }
        Message::DarkThemeChanged(v) => {
            app.dark_theme = v;
            return Task::none();
        }
        Message::RefreshDesigns => {
            designs::refresh_designs(app);
            return render_current(app);
        }

        Message::PickLogoFile => {
            return Task::perform(
                async {
                    rfd::FileDialog::new()
                        .set_title("Выберите изображение логотипа")
                        .add_filter("Изображения", &["png", "jpg", "jpeg", "svg", "webp"])
                        .pick_file()
                        .map(|p| p.to_string_lossy().to_string())
                },
                Message::LogoFilePicked,
            );
        }
        Message::LogoFilePicked(Some(path)) => {
            app.logo_path = path;
            return render_current(app);
        }
        Message::LogoFilePicked(None) => {}

        Message::PickTeamsFile => {
            return Task::perform(
                async {
                    rfd::FileDialog::new()
                        .set_title("Выберите TXT со списком команд")
                        .add_filter("Текст", &["txt"])
                        .pick_file()
                        .map(|p| p.to_string_lossy().to_string())
                },
                Message::TeamsFilePicked,
            );
        }
        Message::TeamsFilePicked(Some(path)) => {
            return Task::perform(
                async move {
                    let content = std::fs::read_to_string(&path)
                        .map_err(|e| format!("Не удалось прочитать файл команд: {}", e))?;

                    let teams: Vec<String> = content
                        .lines()
                        .map(str::trim)
                        .filter(|l| !l.is_empty())
                        .map(ToString::to_string)
                        .collect();

                    if teams.is_empty() {
                        Err("Файл команд пустой".to_string())
                    } else {
                        Ok((path, teams))
                    }
                },
                Message::TeamsLoaded,
            );
        }
        Message::TeamsFilePicked(None) => {}
        Message::TeamsLoaded(Ok((path, teams))) => {
            let count = teams.len() as i32;
            app.team_names = teams;
            app.teams_file_path = path;
            app.start_team = 1;
            app.finish_team = count;
            app.status_message = format!("Загружено команд: {}", count);
            return render_current(app);
        }
        Message::TeamsLoaded(Err(e)) => {
            app.status_message = e;
        }

        Message::SavePdf => {
            return Task::perform(
                async {
                    rfd::FileDialog::new()
                        .set_title("Сохранить PDF")
                        .add_filter("PDF", &["pdf"])
                        .set_file_name("blanks.pdf")
                        .save_file()
                        .map(|p| p.to_string_lossy().to_string())
                },
                Message::PdfPathPicked,
            );
        }
        Message::PdfPathPicked(Some(path)) => {
            let design_name = app.selected_design.clone();
            let inputs = app.to_inputs();
            return Task::perform(
                async move {
                    let bytes = renderer::build_document(design_name, inputs)
                        .and_then(|doc| doc.to_pdf().map_err(|e| e.to_string()))?;
                    std::fs::write(&path, bytes)
                        .map_err(|e| format!("Не удалось сохранить PDF: {}", e))?;
                    Ok(path)
                },
                Message::PdfSaved,
            );
        }
        Message::PdfPathPicked(None) => {}
        Message::PdfSaved(Ok(path)) => {
            app.status_message = format!("PDF сохранён: {}", path);
        }
        Message::PdfSaved(Err(e)) => {
            app.status_message = e;
        }

        Message::SvgLoaded(Ok(svgs)) => {
            app.svg_pages = svgs;
            app.status_message = format!(
                "SVG обновлён: {} стр. | дизайн: {} | шрифт: {}",
                app.svg_pages.len(),
                app.selected_design,
                app.font_name
            );
        }
        Message::SvgLoaded(Err(e)) => {
            app.status_message = format!("Ошибка рендера: {}", e);
            eprintln!("Ошибка генерации SVG: {}", e);
        }
    }
    Task::none()
}

// =============================================
// Запуск приложения
// =============================================
fn main() -> iced::Result {
    iced::application(boot, update, ui::view)
        .title("Генератор бланков")
        .default_font(iced::Font::MONOSPACE)
        .theme(|app: &App| {
            if app.dark_theme {
                Theme::CatppuccinMocha
            } else {
                Theme::CatppuccinLatte
            }
        })
        .window(iced::window::Settings {
            maximized: true,
            size: iced::Size::new(1920.0, 1080.0),
            ..Default::default()
        })
        .run()
}

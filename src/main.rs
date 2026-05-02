// ══════════════════════════════════════════════════════════════════════════════
//  ИМПОРТЫ
// ══════════════════════════════════════════════════════════════════════════════

use iced::border::Radius;
use iced::widget::{button, checkbox, column, container, row, slider, svg, text, text_input};
use iced::{Border, Center, Element, Fill, Font, Length, Task, Theme, color};
use std::path::Path;
use typst_bake::{IntoDict, IntoValue};

// ══════════════════════════════════════════════════════════════════════════════
//  МОДЕЛЬ ДАННЫХ
// ══════════════════════════════════════════════════════════════════════════════

// Всё состояние приложения. Живёт на протяжении всей сессии.
#[derive(Default, Debug)]
struct App {
    // ── параметры бланков ──────────────────────────────
    start_team: i32,
    finish_team: i32,
    cols: i32, // колонок на странице
    rows: i32, // строк на странице
    landscape: bool,
    location: String,
    team_prefix: String,
    font_name: String,

    // ── логотип ────────────────────────────────────────
    logo_path: String,

    // ── список команд из файла ─────────────────────────
    team_names: Vec<String>,
    teams_file_path: String,

    // ── UI-состояние ───────────────────────────────────
    dark_theme: bool,
    status_message: String,
    svg_content: Option<String>, // текущий превью-SVG
}

// Данные, которые передаются в Typst-шаблон при каждом рендере.
// Поля совпадают с sys.inputs в blanks.typ.
#[derive(Clone, Debug, IntoValue, IntoDict)]
struct TypstInputs {
    start_team: i32,
    finish_team: i32,
    cols: i32,
    rows: i32,
    landscape: bool,
    location: String,
    team_prefix: String,
    font_name: String,
    logo_path: String,
    team_names: Vec<String>,
}

impl App {
    fn to_inputs(&self) -> TypstInputs {
        TypstInputs {
            start_team: self.start_team,
            finish_team: self.finish_team,
            cols: self.cols,
            rows: self.rows,
            landscape: self.landscape,
            location: self.location.clone(),
            team_prefix: self.team_prefix.clone(),
            font_name: self.font_name.clone(),
            logo_path: self.logo_path.clone(),
            team_names: self.team_names.clone(),
        }
    }
}

// ══════════════════════════════════════════════════════════════════════════════
//  СООБЩЕНИЯ — события от UI и результаты асинхронных задач
// ══════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
enum Message {
    // ── слайдеры и поля ────────────────────────────────
    StartTeamChanged(i32),
    FinishTeamChanged(i32),
    ColsChanged(i32),
    RowsChanged(i32),
    LandscapeChanged(bool),
    LocationChanged(String),
    TeamPrefixChanged(String),
    FontNameChanged(String),
    LogoPathChanged(String),
    DarkThemeChanged(bool),

    // ── диалог выбора логотипа ─────────────────────────
    PickLogoFile,
    LogoFilePicked(Option<String>),

    // ── загрузка списка команд из TXT ──────────────────
    PickTeamsFile,
    TeamsFilePicked(Option<String>),
    TeamsLoaded(Result<(String, Vec<String>), String>),

    // ── экспорт PDF ───────────────────────────────────
    SavePdf,
    PdfPathPicked(Option<String>),
    PdfSaved(Result<String, String>),

    // ── результат асинхронного рендера ─────────────────
    SvgLoaded(Result<Vec<String>, String>),
}

// ══════════════════════════════════════════════════════════════════════════════
//  РЕНДЕРИНГ — Typst → SVG / PDF
// ══════════════════════════════════════════════════════════════════════════════

// Собирает typst_bake::Document: встраивает логотип (если выбран),
// подставляет виртуальный путь и передаёт все inputs в шаблон.
fn build_document(inputs: TypstInputs) -> Result<typst_bake::Document, String> {
    let mut resolved_inputs = inputs;
    let mut document = typst_bake::document!("blanks.typ");

    if !resolved_inputs.logo_path.trim().is_empty() {
        let host_path = resolved_inputs.logo_path.clone();
        let ext = Path::new(&host_path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("png");
        let virtual_path = format!("runtime/logo.{}", ext);

        // Читаем файл с диска и отдаём Typst как виртуальный файл,
        // потому что Typst не знает про файловую систему Windows.
        let bytes = std::fs::read(&host_path)
            .map_err(|e| format!("Не удалось прочитать логотип: {}", e))?;

        document = document
            .add_file(&virtual_path, bytes)
            .map_err(|e| format!("Не удалось добавить логотип в шаблон: {}", e))?;

        resolved_inputs.logo_path = virtual_path;
    }

    Ok(document.with_inputs(resolved_inputs))
}

// Запускает рендер SVG в фоне и возвращает Task<Message>.
// Вызывается при любом изменении параметров.
fn render_task(inputs: TypstInputs) -> Task<Message> {
    iced::Task::perform(
        async move {
            match build_document(inputs).and_then(|doc| doc.to_svg().map_err(|e| format!("{}", e)))
            {
                Ok(svgs) => Message::SvgLoaded(Ok(svgs)),
                Err(e) => Message::SvgLoaded(Err(e)),
            }
        },
        |msg| msg,
    )
}

// ══════════════════════════════════════════════════════════════════════════════
//  ЖИЗНЕННЫЙ ЦИКЛ — boot / update (iced MVU)
// ══════════════════════════════════════════════════════════════════════════════

fn boot() -> (App, Task<Message>) {
    let app = App {
        start_team: 1,
        finish_team: 20,
        cols: 4,
        rows: 9,
        landscape: false,
        location: String::new(),
        team_prefix: "Команда №".to_string(),
        font_name: "Helvetica Neue".to_string(),
        logo_path: String::new(),
        team_names: Vec::new(),
        teams_file_path: String::new(),
        dark_theme: true,
        status_message: "Готово".to_string(),
        svg_content: None,
    };

    // Запускаем первичный рендер сразу при старте
    let task = render_task(app.to_inputs());
    (app, task)
}

fn update(app: &mut App, msg: Message) -> Task<Message> {
    match msg {
        // ── параметры бланка — любое изменение сразу перерендеривает ──────────
        Message::StartTeamChanged(v) => {
            app.start_team = v;
            if app.start_team > app.finish_team {
                app.finish_team = app.start_team;
            }
            return render_task(app.to_inputs());
        }
        Message::FinishTeamChanged(v) => {
            app.finish_team = v;
            if app.finish_team < app.start_team {
                app.start_team = app.finish_team;
            }
            return render_task(app.to_inputs());
        }
        Message::ColsChanged(v) => {
            app.cols = v;
            return render_task(app.to_inputs());
        }
        Message::RowsChanged(v) => {
            app.rows = v;
            return render_task(app.to_inputs());
        }
        Message::LandscapeChanged(v) => {
            app.landscape = v;
            return render_task(app.to_inputs());
        }
        Message::LocationChanged(v) => {
            app.location = v;
            return render_task(app.to_inputs());
        }
        Message::TeamPrefixChanged(v) => {
            app.team_prefix = v;
            return render_task(app.to_inputs());
        }
        Message::FontNameChanged(v) => {
            app.font_name = v;
            return render_task(app.to_inputs());
        }
        Message::LogoPathChanged(v) => {
            app.logo_path = v;
            return render_task(app.to_inputs());
        }

        // ── диалог выбора логотипа ─────────────────────────────────────────────
        Message::PickLogoFile => {
            return iced::Task::perform(
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
            return render_task(app.to_inputs());
        }
        Message::LogoFilePicked(None) => {}

        // ── загрузка списка команд ─────────────────────────────────────────────
        Message::PickTeamsFile => {
            return iced::Task::perform(
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
            return iced::Task::perform(
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
            return render_task(app.to_inputs());
        }
        Message::TeamsLoaded(Err(e)) => {
            app.status_message = e;
        }

        // ── экспорт PDF ────────────────────────────────────────────────────────
        Message::SavePdf => {
            return iced::Task::perform(
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
            let inputs = app.to_inputs();
            return iced::Task::perform(
                async move {
                    let bytes = build_document(inputs)
                        .and_then(|doc| doc.to_pdf().map_err(|e| format!("{}", e)))?;
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

        // ── прочее ────────────────────────────────────────────────────────────
        Message::DarkThemeChanged(v) => {
            app.dark_theme = v;
        }
        Message::SvgLoaded(Ok(svgs)) => {
            app.svg_content = svgs.into_iter().next();
            app.status_message = "SVG обновлён".to_string();
        }
        Message::SvgLoaded(Err(e)) => {
            app.status_message = format!("Ошибка рендера: {}", e);
            eprintln!("Ошибка генерации SVG: {}", e);
        }
    }
    Task::none()
}

// ══════════════════════════════════════════════════════════════════════════════
//  ИНТЕРФЕЙС — вспомогательные виджеты
// ══════════════════════════════════════════════════════════════════════════════

// Слайдер с числовым значением над ним.
fn slider_with_value<'a, Msg: 'a + Clone, T>(
    range: std::ops::RangeInclusive<T>,
    value: T,
    on_change: impl Fn(T) -> Msg + 'a,
) -> Element<'a, Msg>
where
    T: Copy
        + From<u8>
        + std::cmp::PartialOrd
        + std::fmt::Display
        + num_traits::cast::FromPrimitive
        + Into<f64>
        + 'a,
{
    column![
        text(format!("{}", value))
            .size(16)
            .align_x(Center)
            .width(Fill),
        slider(range, value, on_change),
    ]
    .into()
}

// ══════════════════════════════════════════════════════════════════════════════
//  ИНТЕРФЕЙС — главное представление
// ══════════════════════════════════════════════════════════════════════════════

fn view(app: &App) -> Element<'_, Message> {
    // ── левая панель: все настройки ────────────────────────────────────────────

    let settings = column![
        // · тема
        checkbox(app.dark_theme)
            .label("Тёмная тема")
            .on_toggle(Message::DarkThemeChanged),
        // · диапазон команд
        text("Начальная команда").width(Fill).size(16).center(),
        slider_with_value(1..=60, app.start_team, Message::StartTeamChanged),
        text("Конечная команда").width(Fill).size(16).center(),
        slider_with_value(1..=60, app.finish_team, Message::FinishTeamChanged),
        // · сетка
        text("Колонки").width(Fill).size(16).center(),
        slider_with_value(1..=8, app.cols, Message::ColsChanged),
        text("Строки").width(Fill).size(16).center(),
        slider_with_value(1..=20, app.rows, Message::RowsChanged),
        checkbox(app.landscape)
            .label("Альбомная ориентация")
            .on_toggle(Message::LandscapeChanged),
        // · текстовые поля
        text("Локация").width(Fill).size(16).center(),
        text_input("Название события или места", &app.location).on_input(Message::LocationChanged),
        text("Префикс команды").width(Fill).size(16).center(),
        text_input("Например: Команда №", &app.team_prefix).on_input(Message::TeamPrefixChanged),
        text("Шрифт").width(Fill).size(16).center(),
        text_input("Например: Helvetica Neue", &app.font_name).on_input(Message::FontNameChanged),
        // · логотип
        text("Логотип").width(Fill).size(16).center(),
        row![
            text_input("Пусто = без логотипа", &app.logo_path).on_input(Message::LogoPathChanged),
            button("Выбрать файл").on_press(Message::PickLogoFile),
        ]
        .spacing(8),
        // · список команд из файла
        text("Список команд (TXT, одна команда в строке)")
            .width(Fill)
            .size(16)
            .center(),
        row![
            text_input("Файл не выбран", &app.teams_file_path),
            button("Выбрать TXT").on_press(Message::PickTeamsFile),
        ]
        .spacing(8),
        // · экспорт
        button("Сохранить итоговый PDF").on_press(Message::SavePdf),
    ];

    // ── статусная строка под настройками ──────────────────────────────────────

    let sidebar = column![
        text("Настройки")
            .font(Font::MONOSPACE)
            .size(20)
            .line_height(1.5)
            .width(Fill)
            .center(),
        settings,
        text(format!(
            "Команды: {}..={} | Сетка: {}×{} | Из файла: {}",
            app.start_team,
            app.finish_team,
            app.cols,
            app.rows,
            app.team_names.len()
        )),
        text(&app.status_message),
    ]
    .spacing(8)
    .width(Length::FillPortion(2))
    .spacing(2)
    .padding(6)
    .height(Fill);

    // ── правая панель: превью SVG ──────────────────────────────────────────────

    let preview: Element<_> = match &app.svg_content {
        Some(svg_str) => {
            let handle = iced::widget::svg::Handle::from_memory(svg_str.clone().into_bytes());
            container(svg(handle).width(Fill).height(Fill))
                .width(Fill)
                .height(Fill)
                .center_x(Fill)
                .center_y(Fill)
                .into()
        }
        None => text("⏳ Генерация SVG...").size(24).into(),
    };

    let preview_area = column![preview].width(Length::FillPortion(3)).height(Fill);

    // ── корневой layout ────────────────────────────────────────────────────────

    row![
        container(sidebar).style(|_: &Theme| container::Style {
            border: Border {
                color: color!(100, 100, 100),
                width: 2.0,
                radius: Radius::from(6.0),
            },
            ..Default::default()
        }),
        preview_area,
    ]
    .width(Fill)
    .height(Fill)
    .spacing(16)
    .padding(6)
    .into()
}

// ══════════════════════════════════════════════════════════════════════════════
//  ТОЧКА ВХОДА
// ══════════════════════════════════════════════════════════════════════════════

fn main() -> iced::Result {
    iced::application(boot, update, view)
        .title("Генератор бланков")
        .theme(|app: &App| {
            if app.dark_theme {
                Theme::CatppuccinMocha
            } else {
                Theme::CatppuccinLatte
            }
        })
        .window_size(iced::Size::new(1600.0, 1200.0))
        .centered()
        .run()
}

// =============================================
// UI: отрисовка интерфейса
// =============================================

use crate::app_state::{App, Message};
use iced::border::Radius;
use iced::widget::{
    button, checkbox, column, container, pick_list, row, scrollable, slider, svg, text, text_input,
};
use iced::{Border, Center, Color, Element, Fill, Font, Length, Theme, color};

const SIDEBAR_WIDTH: u16 = 1;
const PREVIEW_WIDTH: u16 = 4;
const ROOT_SPACING: u32 = 0;
const ROOT_PADDING: u16 = 0;
const SECTION_SPACING: u32 = 10;

// =============================================
// Helper: число + слайдер
// =============================================
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
    .spacing(6)
    .into()
}

// =============================================
// Helper: карточка секции
// =============================================
fn section_card<'a>(
    title: &'a str,
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    container(
        column![text(title).font(Font::MONOSPACE).size(16), content.into()]
            .spacing(SECTION_SPACING),
    )
    .padding(10)
    .style(|theme: &Theme| container::Style {
        border: Border {
            color: match theme {
                Theme::CatppuccinMocha => Color::from_rgb(0.35, 0.39, 0.47),
                _ => Color::from_rgb(0.76, 0.76, 0.82),
            },
            width: 1.0,
            radius: Radius::from(8.0),
        },
        ..Default::default()
    })
    .into()
}

// =============================================
// Главный рендер экрана
// Слева: настройки
// Справа: превью
// =============================================
pub fn view(app: &App) -> Element<'_, Message> {
    let rows_range = if app.sheets_mode.is_two() {
        2..=20
    } else {
        1..=20
    };

    let font_selector: Element<_> = pick_list(
        app.available_fonts.clone(),
        Some(app.font_name.clone()),
        Message::FontChanged,
    )
    .into();

    let design_selector: Element<_> = if app.available_designs.is_empty() {
        text("Нет доступных дизайнов в папке templates/designs")
            .size(14)
            .into()
    } else {
        pick_list(
            app.available_designs.clone(),
            Some(app.selected_design.clone()),
            Message::DesignChanged,
        )
        .into()
    };

    let (first_rows, second_rows) = app.sheet_rows();
    let first_page_blanks = app.cols * first_rows;
    let second_page_blanks = app.cols * second_rows;

    let split_info = if app.sheets_mode.is_two() {
        format!(
            "2 листа: лист 1 = {} бланков, лист 2 = {} бланков",
            first_page_blanks, second_page_blanks
        )
    } else {
        format!("1 лист: {} бланков", app.cols * app.rows.max(1))
    };

    let teams_section = section_card(
        "Команды",
        column![
            text("Начальная команда").width(Fill).size(15).center(),
            slider_with_value(1..=60, app.start_team, Message::StartTeamChanged),
            text("Конечная команда").width(Fill).size(15).center(),
            slider_with_value(1..=60, app.finish_team, Message::FinishTeamChanged),
            text("Список команд (TXT, одна команда в строке)")
                .width(Fill)
                .size(15)
                .center(),
            row![
                text_input("Файл не выбран", &app.teams_file_path),
                button("Выбрать TXT").on_press(Message::PickTeamsFile),
            ]
            .spacing(8),
        ]
        .spacing(SECTION_SPACING),
    );

    let layout_section = section_card(
        "Сетка и печать",
        column![
            text("Колонки").width(Fill).size(15).center(),
            slider_with_value(1..=8, app.cols, Message::ColsChanged),
            text("Строки").width(Fill).size(15).center(),
            slider_with_value(rows_range, app.rows, Message::RowsChanged),
            checkbox(app.sheets_mode.is_two())
                .label("Печатать на 2 листа")
                .on_toggle(Message::TwoSheetsChanged),
            checkbox(app.landscape)
                .label("Альбомная ориентация")
                .on_toggle(Message::LandscapeChanged),
            text(split_info).size(14),
        ]
        .spacing(SECTION_SPACING),
    );

    let template_font_section = section_card(
        "Дизайн и шрифт",
        column![
            text("Дизайн бланка").width(Fill).size(15).center(),
            design_selector,
            text("Шрифт").width(Fill).size(15).center(),
            font_selector,
            row![button("Обновить дизайны").on_press(Message::RefreshDesigns)]
                .spacing(8)
                .width(Fill),
        ]
        .spacing(SECTION_SPACING),
    );

    let content_section = section_card(
        "Данные и логотип",
        column![
            text("Локация").width(Fill).size(15).center(),
            text_input("Название события или места", &app.location)
                .on_input(Message::LocationChanged),
            text("Префикс команды").width(Fill).size(15).center(),
            text_input("Например: Команда №", &app.team_prefix)
                .on_input(Message::TeamPrefixChanged),
            text("Логотип").width(Fill).size(15).center(),
            row![
                text_input("Пусто = без логотипа", &app.logo_path)
                    .on_input(Message::LogoPathChanged),
                button("Выбрать файл").on_press(Message::PickLogoFile),
            ]
            .spacing(8),
        ]
        .spacing(SECTION_SPACING),
    );

    let export_section = section_card(
        "Экспорт",
        column![button("Сохранить итоговый PDF").on_press(Message::SavePdf)]
            .spacing(SECTION_SPACING),
    );

    let summary = text(format!(
        "Команды: {}..={} | Сетка: {}×{} | Из файла: {} | Дизайн: {}",
        app.start_team,
        app.finish_team,
        app.cols,
        app.rows,
        app.team_names.len(),
        if app.selected_design.is_empty() {
            "-"
        } else {
            &app.selected_design
        }
    ));

    let sidebar_content = column![
        text("Настройки")
            .font(Font::MONOSPACE)
            .size(20)
            .line_height(1.5)
            .width(Fill)
            .center(),
        checkbox(app.dark_theme)
            .label("Тёмная тема")
            .on_toggle(Message::DarkThemeChanged),
        teams_section,
        layout_section,
        template_font_section,
        content_section,
        export_section,
        summary,
        text(&app.status_message),
    ]
    .spacing(12)
    .width(Fill)
    .height(Fill);

    let sidebar = scrollable(sidebar_content)
        .height(Fill)
        .width(Length::FillPortion(SIDEBAR_WIDTH));

    let preview: Element<_> = if app.svg_pages.is_empty() {
        container(text("Генерация SVG...").size(24))
            .width(Fill)
            .height(Fill)
            .center_x(Fill)
            .center_y(Fill)
            .into()
    } else if app.sheets_mode.is_two() {
        let pages = app.svg_pages.iter().take(2).enumerate().fold(
            column![].spacing(8),
            |col, (idx, svg_str)| {
                let handle = iced::widget::svg::Handle::from_memory(svg_str.as_bytes().to_vec());
                col.push(
                    column![
                        text(format!("Страница {}", idx + 1)).size(14),
                        svg(handle).width(Fill)
                    ]
                    .spacing(4),
                )
            },
        );

        scrollable(container(pages).padding(6).width(Fill))
            .height(Fill)
            .width(Fill)
            .into()
    } else {
        let handle = iced::widget::svg::Handle::from_memory(app.svg_pages[0].as_bytes().to_vec());
        container(svg(handle).width(Fill).height(Fill))
            .width(Fill)
            .height(Fill)
            .center_x(Fill)
            .center_y(Fill)
            .into()
    };

    let preview_area = container(preview)
        .width(Length::FillPortion(PREVIEW_WIDTH))
        .height(Fill)
        .padding(0);

    row![
        container(sidebar).style(|_: &Theme| container::Style {
            border: Border {
                color: color!(100, 100, 100),
                width: 1.0,
                radius: Radius::from(6.0),
            },
            ..Default::default()
        }),
        preview_area,
    ]
    .width(Fill)
    .height(Fill)
    .spacing(ROOT_SPACING)
    .padding(ROOT_PADDING)
    .into()
}

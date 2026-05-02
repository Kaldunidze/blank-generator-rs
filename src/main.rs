use iced::border::Radius;
use iced::widget::{
    button, center_x, center_y, checkbox, column, container, image, radio, rich_text, row,
    scrollable, slider, space, span, text, text_input, toggler,
};
use iced::{Border, Center, Color, Element, Fill, Font, Length, Pixels, Theme, color};

// ============ ФУНКЦИЯ: SLIDER + ТЕКСТ ЗНАЧЕНИЯ ============
pub fn slider_with_value<'a, Message: 'a + Clone, T>(
    range: std::ops::RangeInclusive<T>,
    value: T,
    on_change: impl Fn(T) -> Message + 'a,
) -> Element<'a, Message>
where
    T: Copy
        + From<u8>
        + std::cmp::PartialOrd
        + std::fmt::Display
        + num_traits::cast::FromPrimitive
        + Into<f64>
        + 'a,
    Message: Clone,
{
    column![
        text(format!("{}", value))
            .size(16)
            .align_x(Center)
            .width(Fill),
        slider(range, value, on_change)
    ]
    .into()
}

// ============ СОСТОЯНИЕ ПРИЛОЖЕНИЯ ============
#[derive(Default, Debug)]
struct App {
    width_num: i32,
}

// ============ СООБЩЕНИЯ ============
#[derive(Clone, Copy, Debug)]
enum Message {
    WidthNum(i32),
}

// ============ BOOT ФУНКЦИЯ (создает начальное состояние!) ============
fn boot(_flags: ()) -> (App, iced::Task<Message>) {
    (App { width_num: 5 }, iced::Task::none())
}

// ============ UPDATE ЛОГИКА ============
fn update(app: &mut App, msg: Message) -> iced::Task<Message> {
    match msg {
        Message::WidthNum(c) => {
            app.width_num = c;
        }
    }
    iced::Task::none() // ← Возвращаем Task!
}

// ============ VIEW ЛОГИКА ============
fn view(app: &App) -> Element<'_, Message> {
    //======================================================
    // настройки параметров страницы
    //======================================================
    let param_buttons = column![
        text("width_num").width(Fill).size(20).center(),
        slider_with_value(0..=10, app.width_num, Message::WidthNum),
    ];

    //======================================================

    let menu = column![
        text("Настройки")
            .font(Font::MONOSPACE)
            .size(20)
            .line_height(1.5)
            .width(Fill)
            .center(),
        param_buttons,
        "I am to the right!",
    ]
    .spacing(8)
    .width(Length::FillPortion(2))
    .height(Fill);

    let content = column!["Main content area"]
        .width(Length::FillPortion(3))
        .height(Fill);

    let window: Element<_> = row![
        container(menu).style(|_theme: &Theme| container::Style {
            border: Border {
                color: color!(100, 100, 100),
                width: 2.0,
                radius: Radius::from(12.0),
            },
            ..Default::default()
        }),
        content
    ]
    .width(Fill)
    .height(Fill)
    .spacing(16)
    .into();

    window
}

// ============ ТОЧКА ВХОДА ============
fn main() -> iced::Result {
    iced::application(boot, update, view) // ✅ boot функция первым аргументом!
        .title("Стильное приложение") // ✅ Заголовок здесь!
        .theme(Theme::CatppuccinMocha) // 🎨 Тема
        .window_size(iced::Size::new(900.0, 600.0))
        .centered()
        .run()
}

use iced::border::Radius;
use iced::widget::{column, container, row, slider, svg, text};
use iced::{Border, Center, Element, Fill, Font, Length, Task, Theme, color};

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

#[derive(Default, Debug)]
struct App {
    width_num: i32,
    svg_content: Option<String>,
}

#[derive(Clone, Debug)]
enum Message {
    WidthNum(i32),
    SvgLoaded(Result<Vec<String>, String>),
}

fn boot() -> (App, Task<Message>) {
    let app = App { width_num: 5, svg_content: None };

    let task = iced::Task::perform(
        async move {
            match typst_bake::document!("blanks.typ").to_svg() {
                Ok(svgs) => Message::SvgLoaded(Ok(svgs)),
                Err(e) => Message::SvgLoaded(Err(format!("{}", e))),
            }
        },
        |msg| msg,
    );

    (app, task)
}

fn update(app: &mut App, msg: Message) -> Task<Message> {
    match msg {
        Message::WidthNum(c) => {
            app.width_num = c;
        }
        Message::SvgLoaded(result) => {
            match result {
                Ok(svgs) => {
                    app.svg_content = svgs.into_iter().next();
                }
                Err(e) => {
                    eprintln!("Ошибка генерации SVG: {}", e);
                }
            }
        }
    }
    Task::none()
}

fn view(app: &App) -> Element<'_, Message> {
    let param_buttons = column![
        text("width_num").width(Fill).size(20).center(),
        slider_with_value(0..=10, app.width_num, Message::WidthNum),
    ];

    let content = column![
        match &app.svg_content {
            Some(svg_string) => {
                let handle = iced::widget::svg::Handle::from_memory(svg_string.clone().into_bytes());
                
                let svg_element: Element<'_, Message> = container(
                    svg(handle)
                        .width(Fill)
                        .height(Fill)
                )
                .width(Fill)
                .height(Fill)
                .center_x(Fill)
                .center_y(Fill)
                .into();
                
                svg_element
            }
            None => {
                text("⏳ Генерация SVG...").size(24).into()
            }
        },
    ]
    .width(Length::FillPortion(3))
    .height(Fill);

    let menu = column![
        text("Настройки")
            .font(Font::MONOSPACE)
            .size(20)
            .line_height(1.5)
            .width(Fill)
            .center(),
        param_buttons,
        text(format!("width_num = {}", app.width_num)),
    ]
    .spacing(8)
    .width(Length::FillPortion(2))
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

fn main() -> iced::Result {
    iced::application(boot, update, view)
        .title("Стильное приложение")
        .theme(Theme::CatppuccinMocha)
        .window_size(iced::Size::new(1200.0, 1200.0))
        .centered()
        .run()
}
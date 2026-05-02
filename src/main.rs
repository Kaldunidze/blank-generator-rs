use iced::widget::button;

struct Counter {
    value: i64,
}

enum Message {
    Inc,
    Dec,
}

impl Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Inc => {
                self.value += 1;
            }
            Message::Dec => {
                self.value -= 1;
            }
        }
    }
}

fn main() {
    //use iced::widget::button;

    // let increment = button("+");
    // let decrement = button("-");
}

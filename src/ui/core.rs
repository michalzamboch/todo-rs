use iced::widget::*;
use iced::*;

pub fn init() {
    <Counter as iced::Sandbox>::run(Settings::default()).expect("Could not start the application.")
}

struct Counter {
    count: i32,
    done: bool,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
    Toggle(bool),
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Counter { count: 0, done: false }
    }

    fn title(&self) -> String {
        String::from("Counter app")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Increment => self.count += 1,
            Message::Decrement => self.count -= 1,
            Message::Toggle(completed) => self.done = completed,
        }
    }

    fn view(&self) -> iced::Element<Self::Message> {
        let label = Text::new(format!("Count: {}", self.count));
        let incr = Button::new("Increment").on_press(Message::Increment);
        let decr = Button::new("Decrement").on_press(Message::Decrement);
        let col = Column::new().push(incr).push(label).push(decr);
        let check: Checkbox<'_, Message, Renderer> =
            Checkbox::new("Completed", self.done, Message::Toggle);
        let row = Row::new().push(col).push(check);

        Container::new(row)
            .center_x()
            .center_y()
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }
}

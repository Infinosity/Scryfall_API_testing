use iced::{Element, Theme, Renderer};
use iced::widget::{button, column, text, text_input, Column};

mod api_intf;

fn main() -> iced::Result {
    iced::application(State::default, State::update, State::view).run()
}

// This should have only they elements of the app that change
#[derive(Debug, Default)]
struct State {
    user_input: String,
    //fetched_data: api_intf::Card,
    fetched_data: String,
    loading: bool,
}

#[derive(Debug, Clone)]
enum Message {
    Fetch,
    InputChanged(String),
}

impl State {
    fn update(&mut self, message: Message) {
        match message {
            Message::Fetch => {
                self.fetched_data = self.user_input.clone();
            }
            Message::InputChanged(user_input) => {
                self.user_input = user_input;
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        column![
            text(&self.fetched_data).size(50),
            text_input::<Message, Theme, Renderer>("Enter a card name...", &self.user_input).id("text-input").on_input(Message::InputChanged),
            button("Search").on_press(Message::Fetch),            
        ].into()
    }
} 
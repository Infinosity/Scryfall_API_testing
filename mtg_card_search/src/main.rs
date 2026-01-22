use iced::{Element, Theme, Renderer, Task, Length};
use iced::widget::{Image, button, column, text, text_input, image::Handle};
use anyhow::Result;

mod api_intf;

fn main() -> iced::Result {
    iced::application(State::default, State::update, State::view).run()
}

// State should have only they elements of the app that change
#[derive(Debug, Default)]
struct State {
    user_input: String,
    card_data: api_intf::Card,
}

#[derive(Debug, Clone)]
enum Message {
    Fetch,
    InputChanged(String),
    CardLoaded(Result<api_intf::Card, String>),
}

impl State {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Fetch => {
                let card_name_clone = self.user_input.clone();
                Task::perform(
                    async move {
                        match api_intf::fetch_card(card_name_clone).await {
                            Ok(card) => Ok(card),
                            Err(err) => Err(err.to_string()),
                        }
                    },
                    Message::CardLoaded,
                )
            }
            Message::InputChanged(user_input) => {
                self.user_input = user_input;
                Task::none()
            }
            Message::CardLoaded(Ok(card)) => {
                self.card_data = card;
                Task::none()
            }
            Message::CardLoaded(Err(err)) => {
                eprintln!("Failed to load card: {}", err);
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let image_or_placeholder: Element<'_, Message> = match &self.card_data.image_handle {
            Some(handle) => Image::<Handle>::new(handle.clone())
                .width(Length::Fixed(500.0))
                .height(Length::Fixed(700.0))
                .into(),
            None => text("No Image")
                .size(20)
                .into(),
        };
        column![   
            image_or_placeholder,         
            text(&self.card_data.name).size(30),
            text(&self.card_data.oracle_text).size(20),
            text_input::<Message, Theme, Renderer>("Enter a card name...", &self.user_input).id("text-input").on_input(Message::InputChanged).on_submit(Message::Fetch),
            button("Search").on_press(Message::Fetch),            
        ].into()
    }
} 
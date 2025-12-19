use iced::Center;
use iced::widget::{button, column, text, text_input};
use iced::{Element};
use iced::theme::Theme;
use iced::Renderer;
use std::error::Error;

mod api_intf;
//pub use crate::api_intf;

// Main ///////////////////////
pub fn main() -> iced::Result {
    iced::run(State::update, State::view)
}

// State //////////////////////
#[derive(Default)]
struct State {
    value: i64,
    user_input: String,
    card_info: api_intf::Card,
}

// Messages ///////////////////
#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement,
    Fetch,
    InputChanged(String),
    FetchFinished(Result<(), Box<dyn Error>>),
}

async fn api_call (n: &str, c: &mut api_intf::Card) -> Result<(), Box<dyn Error>> {
    api_intf::fetch_card(n, c).await?;
    Ok(())
}

impl State {
    // Update Logic ///////////////
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -=1;
            }
            Message::Fetch => {
                if !self.user_input.trim().is_empty() {     
                    return none();                                                                    
                }
            }
            Message::InputChanged(user_input) => {
                self.user_input = user_input;
            }
            Message::Done => {
            }
        }
    }
    

    // View Logic /////////////////
    fn view(&self) -> Element<'_, Message> {
        column![
            button("Increment").on_press(Message::Increment),
            text(&self.card_info.name).size(50),
            button("Decrement").on_press(Message::Decrement),
            button("Fetch").on_press(Message::Fetch),
            text_input::<Message, Theme, Renderer>("Enter a card name...", &self.user_input).id("text-input").on_input(Message::InputChanged),
        ]
        .padding(20)
        .align_x(Center).into()
    }
}

// Tests //////////////////////
#[cfg(test)]
mod tests {

    use super::*;
    use iced_test::{Error, simulator};
    use iced_test::selector::id;
    use iced::keyboard;

    #[test]
    fn it_counts() -> Result<(), Error> {
        let mut counter = State { value: 0, user_input: "Enter a card name...".to_string(), card_info: api_intf::Card::default() };
        let mut ui = simulator(counter.view());

        let _ = ui.click("Increment")?;
        let _ = ui.click("Increment")?;
        let _ = ui.click("Decrement")?;

        for message in ui.into_messages() {
            counter.update(message);
        }

        assert_eq!(counter.value, 1);

        let mut ui = simulator(counter.view());
        assert!(ui.find("1").is_ok(), "Counter should display 1!");

        Ok(())
    }

    #[test]
    fn text_works() -> Result<(), Error> {
        let mut state = State {value: 0, user_input: "".to_string(), card_info: api_intf::Card::default()};
        let mut ui = simulator(state.view());

        let _ = ui.click(id("text-input"))?;
        let _ = ui.typewrite("Counterspell");
        let _ = ui.tap_key(keyboard::key::Named::Enter);

        for message in ui.into_messages() {
            state.update(message);
        }

        assert_eq!(state.user_input, "Counterspell");

        Ok(())
    }
}

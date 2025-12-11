use iced::Center;
use iced::widget::{button, column, text, text_input};
use iced::{Element, keyboard};
use iced::theme::Theme;
use iced::Renderer;

// Main ///////////////////////
pub fn main() -> iced::Result {
    iced::run(State::update, State::view)
}

// State //////////////////////
#[derive(Default)]
struct State {
    value: i64,
    user_input: String,
}

// Messages ///////////////////
#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement,
    InputChanged(String),
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
            Message::InputChanged(user_input) => {
                self.user_input = user_input;
            }
        }
    }
    

    // View Logic /////////////////
    fn view(&self) -> Element<'_, Message> {
        column![
            button("Increment").on_press(Message::Increment),
            text(self.value).size(50),
            button("Decrement").on_press(Message::Decrement),
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

    #[test]
    fn it_counts() -> Result<(), Error> {
        let mut counter = State { value: 0, user_input: "Enter a card name...".to_string() };
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
        let mut state = State {value: 0, user_input: "".to_string()};
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

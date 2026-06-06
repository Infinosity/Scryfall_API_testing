// This example demonstrates how to use the tabs widget
//
// This was written by Kaiden42 <gitlab@tinysn.com>

use iced::{
    Element, Font, Length,
    alignment::{Horizontal, Vertical},
    widget::{Column, Container, Text},
};
use iced_aw::ICED_AW_FONT_BYTES;
use iced_aw::{TabLabel, Tabs};

mod settings;
use settings::{SettingsMessage, SettingsTab, TabBarPosition, style_from_index};

const HEADER_SIZE: u32 = 32;
const TAB_PADDING: u16 = 16;
const ICON_BYTES: &[u8] = include_bytes!("./fonts/icons.ttf");
const ICON: Font = Font::with_name("icons");

enum Icon {
    CogAlt,
}

impl From<Icon> for char {
    fn from(icon: Icon) -> Self {
        match icon {
            Icon::CogAlt => '\u{E802}',
        }
    }
}

fn main() -> iced::Result {
    iced::application(
        TabBarExample::default,
        TabBarExample::update,
        TabBarExample::view,
    )
    .font(ICED_AW_FONT_BYTES)
    .font(ICON_BYTES)
    .run()
}

#[derive(Default)]
struct TabBarExample {
    active_tab: TabId,
    settings_tab: SettingsTab,
}
#[derive(Clone, PartialEq, Eq, Debug, Default)]
enum TabId {
    #[default]
    Settings,
}

#[derive(Clone, Debug)]
enum Message {
    TabSelected(TabId),
    Settings(SettingsMessage),
    TabClosed(TabId),
}

impl TabBarExample {
    fn update(&mut self, message: Message) {
        match message {
            Message::TabSelected(selected) => self.active_tab = selected,
            Message::Settings(message) => self.settings_tab.update(message),
            Message::TabClosed(id) => println!("Tab {:?} event hit", id),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let position = self
            .settings_tab
            .settings()
            .tab_bar_position
            .unwrap_or_default();
        let theme = self
            .settings_tab
            .settings()
            .tab_bar_theme
            .unwrap_or_default();

        Tabs::new(Message::TabSelected)
            .tab_icon_position(iced_aw::tabs::Position::Bottom)
            .on_close(Message::TabClosed)
            .push(
                TabId::Settings,
                self.settings_tab.tab_label(),
                self.settings_tab.view(),
            )
            .set_active_tab(&self.active_tab)
            .tab_bar_style(style_from_index(theme))
            .icon_font(ICON)
            .tab_bar_position(match position {
                TabBarPosition::Top => iced_aw::TabBarPosition::Top,
                TabBarPosition::Bottom => iced_aw::TabBarPosition::Bottom,
            })
            .into()
    }
}

trait Tab {
    type Message;

    fn title(&self) -> String;

    fn tab_label(&self) -> TabLabel;

    fn view(&self) -> Element<'_, Self::Message> {
        let column = Column::new()
            .spacing(20)
            .push(Text::new(self.title()).size(HEADER_SIZE))
            .push(self.content())
            .align_x(iced::Alignment::Center);

        Container::new(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .padding(TAB_PADDING)
            .into()
    }

    fn content(&self) -> Element<'_, Self::Message>;
}

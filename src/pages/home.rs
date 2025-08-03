use iced::{
    Alignment, Element, Length,
    widget::{button, column, text},
};

#[derive(Debug, Clone, Copy)]
pub enum HomeMessage {
    GoToAboutPage,
}

#[derive(Debug, Default)]
pub struct HomePage;

const TEXT: [&str; 4] = [
    "Welcome to the Home Page!",
    "This example shows how to manage page navigation in iced with code separated into modules.",
    "Take a look inside the src/pages folder to explore the setup.",
    "Press the button to navigate to the About page.",
];

impl HomePage {
    pub fn update(&mut self, message: HomeMessage) -> Option<HomeMessage> {
        match message {
            HomeMessage::GoToAboutPage => Some(HomeMessage::GoToAboutPage),
        }
    }

    pub fn view(&self) -> Element<HomeMessage> {
        let mut content = vec![text(TEXT[0]).size(30).into()];

        for &line in &TEXT[1..] {
            content.push(text(line).size(18).into());
        }

        column(content)
            .spacing(10)
            .push(
                button(
                    text("About Page")
                        .width(Length::Fill)
                        .align_x(Alignment::Center),
                )
                .width(Length::Fill)
                .on_press(HomeMessage::GoToAboutPage),
            )
            .into()
    }
}

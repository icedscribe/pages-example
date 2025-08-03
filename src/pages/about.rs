use iced::{
    Alignment, Element, Length,
    widget::{button, column, text},
};

#[derive(Debug, Clone, Copy)]
pub enum AboutMessage {
    Back,
    Rate,
}

#[derive(Debug, Default)]
pub struct AboutPage;

const REPOSITORY: &str = "https://github.com/progzone122/iced-pages";

impl AboutPage {
    pub fn update(&mut self, message: AboutMessage) -> Option<AboutMessage> {
        match message {
            AboutMessage::Back => Some(AboutMessage::Back),
            AboutMessage::Rate => {
                if webbrowser::open(REPOSITORY).is_ok() {
                    println!("The {REPOSITORY} link was opened in a default browser");
                } else {
                    eprintln!("Error opening {REPOSITORY} link a in the default browser");
                }
                None
            }
        }
    }

    pub fn view(&self) -> Element<AboutMessage> {
        column![
            text("About page").size(30),
            text("The example was made by DiabloSat with love. Rate a star on the repository :)")
                .size(20),
            column![
                button(
                    text("Rate a star")
                        .width(Length::Fill)
                        .align_x(Alignment::Center)
                )
                .width(Length::Fill)
                .on_press(AboutMessage::Rate),
                button(text("Back").width(Length::Fill).align_x(Alignment::Center))
                    .width(Length::Fill)
                    .on_press(AboutMessage::Back)
            ]
            .spacing(10),
        ]
        .spacing(10)
        .into()
    }
}

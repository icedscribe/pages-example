mod pages;

use iced::widget::{column, container, text};
use iced::{Background, Color, Element, Size, Task};

use pages::about::{AboutMessage, AboutPage};
use pages::home::{HomeMessage, HomePage};

#[derive(Debug)]
enum Page {
    Home(HomePage),
    About(AboutPage),
}
impl Default for Page {
    fn default() -> Self {
        Self::Home(HomePage::default())
    }
}

#[derive(Debug, Default)]
struct App {
    page: Page,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    About(AboutMessage),
    Home(HomeMessage),
}

impl App {
    fn new() -> (Self, Task<Message>) {
        (Self::default(), Task::none())
    }

    fn update(app: &mut Self, message: Message) {
        match (&mut app.page, message) {
            (Page::Home(page), Message::Home(msg)) => {
                if let Some(HomeMessage::GoToAboutPage) = page.update(msg) {
                    app.page = Page::About(AboutPage::default())
                }
            }
            (Page::About(page), Message::About(msg)) => {
                if let Some(AboutMessage::Back) = page.update(msg) {
                    app.page = Page::Home(HomePage::default())
                }
            }
            _ => {}
        }
    }

    fn view(app: &Self) -> Element<Message> {
        let navbar = container(column![text(format!("Current page: {:?}", &app.page))].padding(10))
            .width(iced::Length::Fill)
            .style(|_theme| iced::widget::container::Style {
                background: Some(Background::Color(Color::from_rgb(0.1, 0.2, 0.4))),
                text_color: Some(Color::WHITE),
                ..Default::default()
            });

        let page = match &app.page {
            Page::Home(home_page) => home_page.view().map(Message::Home),
            Page::About(about_page) => about_page.view().map(Message::About),
        };

        column![navbar, column!(page).padding(10)]
            .spacing(10)
            .into()
    }
}

fn main() -> iced::Result {
    iced::application("Pages", App::update, App::view)
        .window_size(Size::new(300.0, 600.0))
        .run_with(App::new)
}

use iced::{Element, Task};

#[derive(Clone, Debug)]
pub struct Event {
    pub name: String,
    pub text: Option<String>,
    pub state: Option<bool>,
    pub index: Option<i64>,
}

#[derive(Clone, Debug, Default)]
pub struct View {
    pub kind: String,
    pub content: String,
    pub checked: bool,
    pub placeholder: String,
    pub event_name: String,
    pub event_index: Option<i64>,
    pub children: Vec<View>,
}

pub trait Application {
    fn title(&self) -> String;
    fn view(&self) -> View;
    fn handle(&mut self, event: Event);
}

struct App<Render> {
    application: Box<dyn Application>,
    view: View,
    render: Render,
}

impl<Render> App<Render>
where
    Render: for<'view> Fn(&'view View) -> Element<'view, Event> + Send + Sync + 'static,
{
    fn boot(
        application: Box<dyn Application>,
        render: Render,
    ) -> (Self, Task<Event>) {
        let view = application.view();
        (
            Self {
                application,
                view,
                render,
            },
            Task::none(),
        )
    }

    fn update(&mut self, event: Event) {
        self.application.handle(event);
        self.view = self.application.view();
    }

    fn view(&self) -> Element<'_, Event> {
        (self.render)(&self.view)
    }
}


pub fn launch<Render>(
    application: Box<dyn Application>,
    render: Render,
) -> Result<bool, iced::Error>
where
    Render: for<'view> Fn(&'view View) -> Element<'view, Event> + Send + Sync + 'static,
{
    let title = application.title();
    let boot = std::sync::Mutex::new(Some((application, render)));
    iced::application(
        move || {
            let (application, render) = boot
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .take()
                .expect("Iced boot callback is invoked once");
            App::boot(application, render)
        },
        App::<Render>::update,
        App::<Render>::view,
    )
    .title(move |_: &App<Render>| title.clone())
    .centered()
    .run()
    .map(|()| true)
}

use iced_ui_host::{Event, View};
use terrane_int_support::IntegerDestination;

fn host_view(view: crate::TerraneNs7IcedUiView) -> View {
    View {
        kind: view.kind,
        content: view.content,
        checked: view.checked,
        placeholder: view.value,
        event_name: view.event_name,
        event_index: view.event_index.map(|index| {
            i64::checked_from_big(&index.as_big()).expect("Iced event index must fit i64")
        }),
        children: view.children.into_vec().into_iter().map(host_view).collect(),
    }
}

fn source_event(event: Event) -> crate::TerraneNs7IcedUiEvent {
    crate::TerraneNs7IcedUiEvent {
        name: event.name,
        text: event.text,
        state: event.state,
        index: event.index.map(Into::into),
    }
}

struct ApplicationAdapter(crate::Application);

impl iced_ui_host::Application for ApplicationAdapter {
    fn title(&self) -> String {
        self.0.title()
    }

    fn view(&self) -> View {
        host_view(self.0.view())
    }

    fn handle(&mut self, event: Event) {
        self.0.handle(source_event(event));
    }
}

fn render(view: &View) -> iced::Element<'_, Event> {
    crate::render_projected(view).into()
}

pub fn launch(application: crate::Application) -> Result<bool, iced::Error> {
    iced_ui_host::launch(Box::new(ApplicationAdapter(application)), render)
}

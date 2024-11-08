use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{
    command, element,
    event::{Command, Event},
    key::Key,
    render_to_frame,
    ui::{Color, Font},
    write, Direction, Element, ElementData, EventResponse, Value,
};

element! {
    /// A text element for displaying static text in the TUI.
    ///
    /// The `Text` element displays text and does not respond to user interactions.
    Text {}
    defaults {}
    fn render(&self, _: usize) -> String {
        self.text.clone()
    }
}

element! {
    style ButtonStyle {
        clicked_color: Color,
        clicked_background: Color,
        clicked_font: Font,
    }

    /// A clickable button element.
    ///
    /// The `Button` element can be clicked, triggering an `on_click` function. Its appearance changes
    /// based on its interaction state, such as being "clicked".
    Button {
        /// A callback function executed when the button is clicked. use `arc!` to use function
        pub on_click: Arc<Mutex<dyn FnMut(&mut Button)>>,
        pub event_response: EventResponse,
    }

    defaults {
        on_click: Arc::new(Mutex::new(|_btn: &mut Button<'_>| {})),
        event_response: command!(
            Command::Render(2),
            Command::Sleep(120)
        ),
    }

    fn render(&self, state: usize) -> String {
        if state == 2 {
            return write!(self, clicked, self.text);
        }
        write!((self, state), self.text)
    }

    fn event(&mut self, event: Event) -> EventResponse {
        match event {
            Event::Key(k) => {
                if k == Key::Enter {
                    let mut btn = self.clone();
                    let mut on_click = self.on_click.lock().unwrap();
                    (on_click)(&mut btn);
                    drop(on_click);
                    *self = btn;
                    return self.event_response.clone();
                }
            }
            _ => {}
        }

        EventResponse::None
    }
}

element! {
    /// A container element that can hold multiple child elements and handle directional key input.
    ///
    /// The `Div` element serves as a container for other elements, allowing navigation between them
    /// using directional keys.
    Div {
        pub keybinds: HashMap<Key, Direction>
    }

    defaults {
        keybinds: HashMap::from([
            (Key::Up, Direction::Up),
            (Key::Down, Direction::Down),
            (Key::Left, Direction::Left),
            (Key::Right, Direction::Right),
        ])
    }

    fn render(&self, state: usize) -> String {
        let mut frame = crate::create_frame(self.width, self.height);
        for (i, child) in (&self.children).iter().enumerate() {
            if i==self.child {
                render_to_frame(state, &mut frame, child);
            } else {
                render_to_frame(0, &mut frame, child);
            }
        }
        frame.join("\n")
    }

    fn event(&mut self, event: Event) -> EventResponse {
        match event.clone() {
            Event::Key(k) => {
                if let Some(direction) = self.keybinds.get(&k) {
                    self.child = crate::closest_component(&self.children, self.child, direction.clone());
                } else if let Some(child) = self.get_child() {
                    let res = child.event(event.clone());
                    match res.clone() {
                        EventResponse::UpdateElementById(id, elem) => {
                            for old in &mut self.children {
                                if old.get_id() == id {
                                    *old = elem.clone();
                                }
                            }
                        }
                        EventResponse::UpdateSelf(elem) => {
                            *child = elem;
                        }
                        _ => {}
                    }
                    return res;
                }
            }
            _ => {}
        }

        EventResponse::None
    }
}

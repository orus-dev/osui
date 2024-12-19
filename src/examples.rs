pub mod counter_example {
    use crate::prelude::*;

    pub fn app() -> Element {
        let count = State::new(0);

        rsx! {
            @SetStyle(styling())

            button {
                class: "btn",

                on_click: fn(_, _, _) @count {
                    count += 1;
                },

                "The current count is: {count}"
            }
        }
    }

    pub fn styling() -> Css {
        css! {
            "btn" {
                outline: true,
                outline_color: Red
            }
        }
    }
}

pub mod todo_example {
    use crate::prelude::*;
    pub fn app() -> Element {
        let todo = vec!["Todo", "Hovering", "Completed", "Foo", "Bar"];

        rsx! {
            @SetStyle(styling())

            text { class: "title", static "Todo list app made with OSUI" }
            text { class: "title", "{}", "-".repeat(crossterm::terminal::size().unwrap().0 as usize) }
            for (t in todo) {
                ersx!(button { class: "todo", on_click: fn(btn: &mut Button, _, _) {
                    btn.class = if btn.class == "todo" {
                        "complete"
                    } else {
                        "todo"
                    };
                }, "{t}" } )
            }
        }
    }

    pub fn styling() -> Css {
        css! {
            "todo" {
                x: 0 px,
                y: Auto,
                color: Red,
            }
            "todo": "hover" {
                color: Blue,
            }
            "complete" {
                x: 0 px,
                y: Auto,
                color: Green,
            }
            "complete": "hover" {
                color: Cyan
            }
            "title" {
                x: 0 px,
                y: Auto,
            }
            "title": "hover" {
                color: Blue
            }
        }
    }
}

pub mod login_example {
    use crate::prelude::*;

    pub fn second_screen(usr: String, psw: String) -> Element {
        rsx! {
            text { "Username: {usr}\nPassword: {psw}" }
        }
    }

    pub fn app() -> Element {
        rsx! {
            @SetStyle(styling())

            input { class: "input", id: "usr", on_click: fn(_, _, document) {
                let root = document.get_root::<Div>();
                root.children.set_index(1);
            }, "username: " }
            input { class: "input", id: "psw", on_click: fn(psw: &mut Input, _, document) {
                if let Some(usr) = document.get_element_by_id::<Input>("usr") {
                    document.draw(second_screen(usr.text.clone(), psw.text.clone()));
                }
            }, "password: " }
        }
    }

    pub fn styling() -> Css {
        css! {
            "input" {
                width: 50 px,
                outline: true,
                y: Auto,
                x: 0 px,
                caret: String::new(),
            }
            "input": "hover" {
                outline_color: Red,
                caret: String::from("█"),
            }
        }
    }
}

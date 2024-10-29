use std::collections::HashMap;

use crate::{
    create_frame,
    key::{self, KeyKind},
    utils::{closest_component, render_to_frame},
    Component, UpdateResponse,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Style {
    pub bg: Color,
    pub fg: Color,
    pub outline: Color,
    pub font: Font,

    pub hover_bg: Color,
    pub hover_fg: Color,
    pub hover_outline: Color,
    pub hover_font: Font,
    pub hover_cursor_fg: Color,
    pub hover_cursor_bg: Color,

    pub clicked_bg: Color,
    pub clicked_fg: Color,
    pub clicked_outline: Color,
    pub clicked_font: Font,

    pub selected_bg: Color,
    pub selected_fg: Color,
    pub selected_font: Font,

    pub cursor_fg: Color,
    pub cursor_bg: Color,

    pub is_active: bool,
}

impl Default for Style {
    fn default() -> Style {
        Style {
            bg: Color::None,
            fg: Color::None,
            outline: Color::None,
            font: Font::None,

            hover_bg: Color::None,
            hover_fg: Color::None,
            hover_outline: Color::None,
            hover_font: Font::None,
            hover_cursor_fg: Color::None,
            hover_cursor_bg: Color::None,

            clicked_bg: Color::None,
            clicked_fg: Color::None,
            clicked_outline: Color::None,
            clicked_font: Font::None,

            selected_bg: Color::None,
            selected_fg: Color::None,
            selected_font: Font::None,
            cursor_fg: Color::None,
            cursor_bg: Color::None,

            is_active: false,
        }
    }
}

impl Style {
    pub fn get(&self) -> String {
        if self.is_active {
            format!(
                "{}{}{}",
                self.fg.prioritize(&self.hover_fg).ansi(),
                self.bg.prioritize(&self.hover_bg).ansi_bg(),
                self.font.prioritize(&self.hover_font).ansi()
            )
        } else {
            format!(
                "{}{}{}",
                self.fg.ansi(),
                self.bg.ansi_bg(),
                self.font.ansi()
            )
        }
    }

    pub fn get_outline(&self) -> String {
        if self.is_active {
            self.outline.prioritize(&self.hover_outline).ansi()
        } else {
            self.outline.ansi()
        }
    }

    pub fn get_clicked(&self) -> String {
        format!(
            "{}{}{}",
            self.fg.prioritize(&self.clicked_fg).ansi(),
            self.bg.prioritize(&self.clicked_bg).ansi_bg(),
            self.font.prioritize(&self.clicked_font).ansi()
        )
    }

    pub fn get_selected(&self) -> String {
        format!(
            "{}{}{}",
            self.fg.prioritize(&self.selected_fg).ansi(),
            self.bg.prioritize(&self.selected_bg).ansi_bg(),
            self.font.prioritize(&self.selected_font).ansi()
        )
    }

    pub fn get_cursor(&self) -> String {
        if self.is_active {
            format!(
                "{}{}",
                self.cursor_fg.prioritize(&self.hover_cursor_fg).ansi(),
                self.cursor_bg.prioritize(&self.hover_cursor_bg).ansi_bg(),
            )
        } else {
            format!("{}{}", self.cursor_fg.ansi(), self.cursor_bg.ansi_bg())
        }
    }

    pub fn write(&self, s: &str) -> String {
        format!("{}{}\x1b[0m", self.get(), s)
    }

    pub fn write_outline(&self, s: &str) -> String {
        format!("{}{}\x1b[0m", self.get_outline(), s)
    }

    pub fn write_clicked(&self, s: &str) -> String {
        format!("{}{}\x1b[0m", self.get_clicked(), s)
    }

    pub fn write_selected(&self, s: &str) -> String {
        format!("{}{}\x1b[0m", self.get_selected(), s)
    }

    pub fn write_cursor(&self, s: &str) -> String {
        format!("{}{}\x1b[0m", self.get_cursor(), s)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Font {
    None,
    Bold,
    Underline,
    Italic,
    Reverse,
    Strike,
    Mul(Vec<Font>),
}

impl Font {
    pub fn ansi(&self) -> String {
        String::from(match self {
            Font::None => "",
            Font::Bold => "\x1b[1m",
            Font::Underline => "\x1b[4m",
            Font::Italic => "\x1b[3m",
            Font::Reverse => "\x1b[7m",
            Font::Strike => "\x1b[9m",
            Font::Mul(v) => {
                let mut s = String::new();
                for n in v {
                    s += n.ansi().as_str();
                }
                return s;
            }
        })
    }

    pub fn prioritize<'a>(&'a self, secondary: &'a Font) -> &Font {
        if secondary == &Font::None {
            self
        } else {
            secondary
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    None,
    Rgb(u8, u8, u8),
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl Color {
    pub fn ansi(&self) -> String {
        String::from(match self {
            Color::None => "",
            Self::Rgb(r, g, b) => {
                return format!("\x1b[38;2;{r};{g};{b}m");
            }
            Color::Black => "\x1b[30m",
            Color::Red => "\x1b[31m",
            Color::Green => "\x1b[32m",
            Color::Yellow => "\x1b[33m",
            Color::Blue => "\x1b[34m",
            Color::Magenta => "\x1b[35m",
            Color::Cyan => "\x1b[36m",
            Color::White => "\x1b[37m",
        })
    }

    pub fn ansi_bg(&self) -> String {
        String::from(match self {
            Color::None => "",
            Self::Rgb(r, g, b) => {
                return format!("\x1b[48;2;{r};{g};{b}m");
            }
            Color::Black => "\x1b[40m",
            Color::Red => "\x1b[41m",
            Color::Green => "\x1b[42m",
            Color::Yellow => "\x1b[43m",
            Color::Blue => "\x1b[44m",
            Color::Magenta => "\x1b[45m",
            Color::Cyan => "\x1b[46m",
            Color::White => "\x1b[47m",
        })
    }

    pub fn prioritize<'a>(&'a self, secondary: &'a Color) -> &Color {
        if secondary == &Color::None {
            self
        } else {
            secondary
        }
    }
}

pub fn div() -> Component {
    let mut component = Component::new();

    component.binds = HashMap::from([
        (KeyKind::Up, String::from("up")),
        (KeyKind::Down, String::from("down")),
        (KeyKind::Left, String::from("left")),
        (KeyKind::Right, String::from("right")),
    ]);

    fn update(this: &mut Component, k: key::Key) -> UpdateResponse {
        if let Some(v) = this.binds.get(&k.kind) {
            match v.as_str() {
                "up" => {
                    this.child =
                        closest_component(&this.children, this.child, crate::utils::Direction::Up);
                }

                "down" => {
                    this.child = closest_component(
                        &this.children,
                        this.child,
                        crate::utils::Direction::Down,
                    );
                }

                "left" => {
                    this.child = closest_component(
                        &this.children,
                        this.child,
                        crate::utils::Direction::Left,
                    );
                }

                "right" => {
                    this.child = closest_component(
                        &this.children,
                        this.child,
                        crate::utils::Direction::Right,
                    );
                }

                _ => {}
            }
        } else {
            if let Some(child) = this.get_child() {
                return (child.update)(child, k);
            }
        }

        UpdateResponse::None
    }

    fn render(this: &mut Component) -> String {
        let mut frame: Vec<String> = create_frame!(this.width, this.height);
        for (i, child) in &mut this.children.iter_mut().enumerate() {
            if child.width == 0 {
                child.width = this.width;
            }
            if child.height == 0 {
                child.height = this.height;
            }
            child.style.is_active = this.style.is_active && i == this.child;
            render_to_frame(&mut frame, child);
        }
        frame.join("\n")
    }

    fn tick(this: &mut Component, i: usize) {
        if let Some(child) = this.get_child() {
            return (child.tick)(child, i);
        }
    }

    component.update = update;
    component.render = render;
    component.tick = tick;
    component
}

pub fn text() -> Component {
    let mut component = Component::new();

    fn update(_: &mut Component, _: key::Key) -> UpdateResponse {
        UpdateResponse::None
    }

    fn render(s: &mut Component) -> String {
        s.expr.clone()
    }

    component.update = update;
    component.render = render;
    component
}

pub fn button() -> Component {
    let mut component = Component::new();

    component.binds = HashMap::from([(key::KeyKind::Enter, String::from("click"))]);

    fn update(this: &mut Component, k: key::Key) -> UpdateResponse {
        if let Some(v) = this.binds.get(&k.kind) {
            if v == "click" {
                if this.toggle {
                    this.clicked = !this.clicked;
                    (this.on_click)(this);
                } else {
                    return UpdateResponse::Tick(vec![1, 100]);
                }
            }
        }
        UpdateResponse::None
    }

    fn tick(this: &mut Component, i: usize) {
        if i == 0 {
            this.clicked = true;
            (this.on_click)(this);
        } else if i == 1 {
            this.clicked = false;
            (this.on_click)(this);
        }
    }

    fn render(this: &mut Component) -> String {
        if this.clicked {
            return this.style.write_clicked(&this.expr);
        }
        this.style.write(&this.expr)
    }

    component.update = update;
    component.render = render;
    component.tick = tick;
    component
}

pub fn tab() -> Component {
    let mut component = Component::new();

    component.binds = HashMap::from([
        (key::KeyKind::Tab, String::from("next")),
        (key::KeyKind::ShiftTab, String::from("previous")),
    ]);

    component.style.clicked_fg = crate::ui::Color::Red;

    fn update(this: &mut Component, k: key::Key) -> UpdateResponse {
        if let Some(v) = this.binds.get(&k.kind) {
            match v.as_str() {
                "next" => {
                    if this.child + 1 < this.children.len() {
                        this.child += 1;
                    } else {
                        this.child = 0;
                    }
                }
                "previous" => {
                    if this.child == 0 {
                        this.child = this.children.len() - 1;
                    } else {
                        this.child -= 1;
                    }
                }
                _ => {}
            }
        } else {
            if let Some(child) = this.get_child() {
                return (child.update)(child, k);
            } else {
            }
        }
        UpdateResponse::None
    }

    fn render(this: &mut Component) -> String {
        let mut frame: Vec<String> = create_frame!(this.width, this.height - 1);
        let w = this.width;
        let h = this.height;
        if let Some(child) = this.get_child() {
            if child.width == 0 {
                child.width = w;
            }
            if child.height == 0 {
                child.height = h;
            }
            child.style.is_active = true;
            render_to_frame(&mut frame, child);
        }
        let mut v: String = " ".repeat((this.width / 2) - this.children.len());
        for (i, _) in (&this.children).into_iter().enumerate() {
            if i == this.child {
                v += this.style.write_clicked("*").as_str()
            } else {
                v += this.style.write("*").as_str()
            }
        }
        format!("{}\n{}", v, frame.join("\n"))
    }

    fn tick(this: &mut Component, i: usize) {
        if let Some(child) = this.get_child() {
            return (child.tick)(child, i);
        }
    }

    component.update = update;
    component.tick = tick;
    component.render = render;
    component
}

pub fn menu() -> Component {
    let mut component = Component::new();

    component.style.selected_bg = Color::Magenta;
    component.style.selected_fg = Color::Black;
    component.style.cursor_fg = Color::Magenta;

    fn render(this: &mut Component) -> String {
        let mut res: Vec<String> = Vec::new();

        for (i, item) in this.children.iter_mut().enumerate() {
            if i == this.child {
                res.push(format!(
                    "{}{}",
                    this.style.write_cursor("> "),
                    this.style.write_selected(&item.expr)
                ));
            } else {
                res.push(format!("  {}", &item.expr));
            }
        }

        res.join(&this.style.write("\n"))
    }

    fn update(this: &mut Component, k: key::Key) -> UpdateResponse {
        if k.kind == key::KeyKind::Down {
            if this.children.len() > this.child + 1 {
                this.child += 1;
            } else {
                this.child = 0;
            }
        } else if k.kind == key::KeyKind::Up {
            if this.child > 0 {
                this.child -= 1;
            } else {
                this.child = this.children.len() - 1;
            }
        } else if k.kind == key::KeyKind::Enter {
            (this.on_click)(this)
        }
        UpdateResponse::None
    }

    component.render = render;
    component.update = update;
    component
}

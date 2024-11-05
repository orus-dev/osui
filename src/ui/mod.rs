/// The `ui` module provides user interface components and utilities for building
/// a text-based user interface (TUI). It includes predefined styles and elements
/// such as text, buttons, and containers.

pub mod styles;
pub use styles::*;
pub mod elements;
pub use elements::*;

/// Creates a new `Text` element.
///
/// The `Text` element displays static text in the TUI, suitable for labels or headers.
///
/// # Returns
///
/// A `Box<Text>` containing a new `Text` element instance.
pub fn text() -> Box<Text> {
    Box::new(Text::new())
}

/// Creates a new `Button` element with a customized style.
///
/// The `Button` element responds to user clicks, triggering an action. This function
/// customizes the button's clicked background and foreground colors.
///
/// # Returns
///
/// A `Box<Button>` containing a new `Button` element instance.
pub fn button() -> Box<Button> {
    let mut btn = Button::new();
    btn.style.clicked_bg = styles::Color::White;
    btn.style.clicked_fg = styles::Color::Black;
    Box::new(btn)
}

/// Creates a new `Div` container element.
///
/// The `Div` element serves as a container for other UI elements, enabling navigation
/// and grouping of child elements. It supports directional navigation using key bindings.
///
/// # Returns
///
/// A `Box<Div>` containing a new `Div` element instance.
pub fn div() -> Box<Div> {
    Box::new(Div::new())
}

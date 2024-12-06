#[macro_export]
macro_rules! check_expr {
    (|| $code:block) => {
        $crate::Handler::new(move |_, _, _| $code)
    };
    (|$($inner:tt)*| $code:block) => {
        $crate::Handler::new(move |$($inner)*| $code)
    };
    ($expr:expr) => {
        $expr
    };
}
#[macro_export]
macro_rules! parse_rsx_param {
    // Data fields
    ($elem:expr, @$name:ident: $type:ty; $($rest:tt)*) => {
        if $elem.children.is_none() {$elem.children = $crate::Children::Children(Vec::new(), 0)}
        if let $crate::Children::Children(children, _) = &mut $elem.children {
            let mut $name = $crate::ui::data_holder::<$type>();
            $name.id = stringify!($name);
            children.push($name);
        }
        osui::parse_rsx_param!($elem, $($rest)*)
    };

    ($elem:expr, for ($($for:tt)*) $code:block $($rest:tt)*) => {
        if $elem.children.is_none() {$elem.children = $crate::Children::Children(Vec::new(), 0)}
        if let $crate::Children::Children(children, _) = &mut $elem.children {
            for $($for)* {
                children.push($code)
            }
        }
        osui::parse_rsx_param!($elem, $($rest)*);
    };

    ($elem:expr, $($k:ident).+: fn($($params:tt)*) $code:block $(, $($rest:tt)*)?) => {
        $elem.$($k).+ = $crate::Handler::new(|$($params)*| $code);
        osui::parse_rsx_param!($elem, $($($rest)*)?);
    };

    ($elem:expr, $($k:ident).+: $v:expr $(, $($rest:tt)*)?) => {
        $elem.$($k).+ = $crate::check_expr!($v);
        osui::parse_rsx_param!($elem, $($($rest)*)?);
    };
    ($elem:expr, $p:path) => {
        $p;
    };
    ($elem:expr, $($k:ident).+, $($rest:tt)*) => {
        $elem.$($k).+ = true;
        osui::parse_rsx_param!($elem, $($rest)*);
    };
    ($elem:expr, $($k:ident).+., $($rest:tt)*) => {
        $elem.$($k).+.;
        osui::parse_rsx_param!($elem, $($rest)*);
    };

    ($elem:expr, $($k:ident).+.: $v:expr) => {
        $elem.$($k).+. = $crate::check_expr!($v);
    };
    ($elem:expr, $($k:ident).+.: $v:expr, $(, $($rest:tt)*)?) => {
        $elem.$($k).+. = $crate::check_expr!($v);
        osui::parse_rsx_param!($elem, $($($rest)*)?);
    };

    ($elem:expr, $code:block $($rest:tt)*) => {
        if $elem.children.is_none() {$elem.children = $crate::Children::Children(Vec::new(), 0)}
        if let $crate::Children::Children(children, _) = &mut $elem.children {
            children.push($code);
        }
        osui::parse_rsx_param!($elem, $($rest)*);
    };

    ($elem:expr, $elem_path:path { $($inner:tt)* } $($rest:tt)*) => {
        if $elem.children.is_none() {$elem.children = $crate::Children::Children(Vec::new(), 0)}
        if let $crate::Children::Children(children, _) = &mut $elem.children {
            children.push(osui::ersx!($elem_path { $($inner)* }));
        }
        osui::parse_rsx_param!($elem, $($rest)*)
    };

    ($elem:expr, $elem_path:path as $t:ty { $($inner:tt)* } $($rest:tt)*) => {
        if $elem.children.is_none() {$elem.children = $crate::Children::Children(Vec::new(), 0)}
        if let $crate::Children::Children(children, _) = &mut $elem.children {
            children.push(osui::ersx!($elem_path as $t { $($inner)* }));
        }
        osui::parse_rsx_param!($elem, $($rest)*)
    };

    ($elem:expr, $text:expr) => {
        $elem.children.set_text_force(&format!($text))
    };

    ($elem:expr, $text:expr, $($inner:tt)*) => {
        $elem.children.set_text_force(&format!($text, $($inner)*))
    };

    ($elem:expr, ) => {};
    ($elem:expr) => {};
}

#[macro_export]
macro_rules! ersx {
    ($elem:path { $($inner:tt)* }) => {{
        #[allow(unused_mut)]
        let mut elem = $elem();
        $crate::parse_rsx_param!(elem, $($inner)*);
        elem as $crate::Element
    }};

    ($elem:path as $t:ty { $($inner:tt)* }) => {{
        #[allow(unused_mut)]
        let mut elem = paste::paste!{$elem::<$t>}();
        $crate::parse_rsx_param!(elem, $($inner)*);
        elem as $crate::Element
    }};
}

/// Makes a div and puts elements into it
///
/// # Example
/// ```
/// rsx! {
///     button { class: "btn", "Click me!" }
/// }
/// ```
///
/// # Returns
/// A `osui::Element` - Which is a `Box<dyn osui::ElementWidget>`
#[macro_export]
macro_rules! rsx {
    ($($inner:tt)*) => {
        $crate::ersx!{ $crate::ui::div { $($inner)* } }
    };

}

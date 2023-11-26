use std::borrow::Cow;

use rsh_color_config::{Alignment, StyleComputer};
use rsh_protocol::{ShellError, Value};
use rsh_table::{string_width, TextStyle};
use ratatui::{
    buffer::Buffer,
    style::{Color, Modifier, Style},
    text::Span,
};

use crate::rsh_common::{truncate_str, rshColor, rshStyle, RshText};

pub fn set_span(
    buf: &mut Buffer,
    (x, y): (u16, u16),
    text: &str,
    style: Style,
    max_width: u16,
) -> u16 {
    let mut text = Cow::Borrowed(text);
    let mut text_width = string_width(&text);
    if text_width > max_width as usize {
        let mut s = text.into_owned();
        truncate_str(&mut s, max_width as usize);
        text = Cow::Owned(s);
        text_width = max_width as usize;
    }

    let span = Span::styled(text.as_ref(), style);
    buf.set_span(x, y, &span, text_width as u16);

    text_width as u16
}

pub fn lookup_tui_color(style_computer: &StyleComputer, key: &str) -> Style {
    let rsh_style = style_computer.compute(key, &Value::nothing(rsh_protocol::Span::unknown()));
    rsh_style_to_tui(rsh_style)
}

pub fn rsh_style_to_tui(style: rshStyle) -> ratatui::style::Style {
    let mut out = ratatui::style::Style::default();
    if let Some(clr) = style.background {
        out.bg = rsh_ansi_color_to_tui_color(clr);
    }

    if let Some(clr) = style.foreground {
        out.fg = rsh_ansi_color_to_tui_color(clr);
    }

    if style.is_blink {
        out.add_modifier |= Modifier::SLOW_BLINK;
    }

    if style.is_bold {
        out.add_modifier |= Modifier::BOLD;
    }

    if style.is_dimmed {
        out.add_modifier |= Modifier::DIM;
    }

    if style.is_hidden {
        out.add_modifier |= Modifier::HIDDEN;
    }

    if style.is_italic {
        out.add_modifier |= Modifier::ITALIC;
    }

    if style.is_reverse {
        out.add_modifier |= Modifier::REVERSED;
    }

    if style.is_underline {
        out.add_modifier |= Modifier::UNDERLINED;
    }

    out
}

pub fn rsh_ansi_color_to_tui_color(clr: rshColor) -> Option<ratatui::style::Color> {
    use rshColor::*;

    let clr = match clr {
        Black => Color::Black,
        DarkGray => Color::DarkGray,
        Red => Color::Red,
        LightRed => Color::LightRed,
        Green => Color::Green,
        LightGreen => Color::LightGreen,
        Yellow => Color::Yellow,
        LightYellow => Color::LightYellow,
        Blue => Color::Blue,
        LightBlue => Color::LightBlue,
        Magenta => Color::Magenta,
        LightMagenta => Color::LightMagenta,
        Cyan => Color::Cyan,
        LightCyan => Color::LightCyan,
        White => Color::White,
        Fixed(i) => Color::Indexed(i),
        Rgb(r, g, b) => ratatui::style::Color::Rgb(r, g, b),
        LightGray => Color::Gray,
        LightPurple => Color::LightMagenta,
        Purple => Color::Magenta,
        Default => return None,
    };

    Some(clr)
}

pub fn text_style_to_tui_style(style: TextStyle) -> ratatui::style::Style {
    let mut out = ratatui::style::Style::default();
    if let Some(style) = style.color_style {
        out = rsh_style_to_tui(style);
    }

    out
}

// This is identical to the same function in rsh-explore/src/rsh_common
pub fn make_styled_string(
    style_computer: &StyleComputer,
    text: String,
    value: Option<&Value>, // None represents table holes.
    float_precision: usize,
) -> RshText {
    match value {
        Some(value) => {
            match value {
                Value::Float { .. } => {
                    // set dynamic precision from config
                    let precise_number = match convert_with_precision(&text, float_precision) {
                        Ok(num) => num,
                        Err(e) => e.to_string(),
                    };
                    (precise_number, style_computer.style_primitive(value))
                }
                _ => (text, style_computer.style_primitive(value)),
            }
        }
        None => {
            // Though holes are not the same as null, the closure for "empty" is passed a null anyway.
            (
                text,
                TextStyle::with_style(
                    Alignment::Center,
                    style_computer.compute("empty", &Value::nothing(rsh_protocol::Span::unknown())),
                ),
            )
        }
    }
}

fn convert_with_precision(val: &str, precision: usize) -> Result<String, ShellError> {
    // val will always be a f64 so convert it with precision formatting
    let val_float = match val.trim().parse::<f64>() {
        Ok(f) => f,
        Err(e) => {
            return Err(ShellError::GenericError(
                format!("error converting string [{}] to f64", &val),
                "".to_string(),
                None,
                Some(e.to_string()),
                Vec::new(),
            ));
        }
    };
    Ok(format!("{val_float:.precision$}"))
}

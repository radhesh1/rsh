use crate::{
    rsh_style::{color_from_hex, lookup_style},
    parse_rsh_style, RshStyle,
};
use nu_ansi_term::Style;
use rsh_protocol::{Record, Value};
use std::collections::HashMap;

pub fn lookup_ansi_color_style(s: &str) -> Style {
    if s.starts_with('#') {
        color_from_hex(s)
            .ok()
            .and_then(|c| c.map(|c| c.normal()))
            .unwrap_or_default()
    } else if s.starts_with('{') {
        color_string_to_rsh_style(s.to_string())
    } else {
        lookup_style(s)
    }
}

pub fn get_color_map(colors: &HashMap<String, Value>) -> HashMap<String, Style> {
    let mut hm: HashMap<String, Style> = HashMap::new();

    for (key, value) in colors {
        parse_map_entry(&mut hm, key, value);
    }

    hm
}

fn parse_map_entry(hm: &mut HashMap<String, Style>, key: &str, value: &Value) {
    let value = match value {
        Value::String { val, .. } => Some(lookup_ansi_color_style(val)),
        Value::Record { val, .. } => get_style_from_value(val).map(parse_rsh_style),
        _ => None,
    };
    if let Some(value) = value {
        hm.entry(key.to_owned()).or_insert(value);
    }
}

fn get_style_from_value(record: &Record) -> Option<RshStyle> {
    let mut was_set = false;
    let mut style = RshStyle::from(Style::default());
    for (col, val) in record {
        match col.as_str() {
            "bg" => {
                if let Value::String { val, .. } = val {
                    style.bg = Some(val.clone());
                    was_set = true;
                }
            }
            "fg" => {
                if let Value::String { val, .. } = val {
                    style.fg = Some(val.clone());
                    was_set = true;
                }
            }
            "attr" => {
                if let Value::String { val, .. } = val {
                    style.attr = Some(val.clone());
                    was_set = true;
                }
            }
            _ => (),
        }
    }

    if was_set {
        Some(style)
    } else {
        None
    }
}

fn color_string_to_rsh_style(color_string: String) -> Style {
    // eprintln!("color_string: {}", &color_string);
    if color_string.is_empty() {
        return Style::default();
    }

    let rsh_style = match rsh_json::from_str::<RshStyle>(&color_string) {
        Ok(s) => s,
        Err(_) => return Style::default(),
    };

    parse_rsh_style(rsh_style)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nu_ansi_term::{Color, Style};
    use rsh_protocol::{Span, Value};

    #[test]
    fn test_color_string_to_rsh_style_empty_string() {
        let color_string = String::new();
        let style = color_string_to_rsh_style(color_string);
        assert_eq!(style, Style::default());
    }

    #[test]
    fn test_color_string_to_rsh_style_valid_string() {
        let color_string = r#"{"fg": "black", "bg": "white", "attr": "b"}"#.to_string();
        let style = color_string_to_rsh_style(color_string);
        assert_eq!(style.foreground, Some(Color::Black));
        assert_eq!(style.background, Some(Color::White));
        assert!(style.is_bold);
    }

    #[test]
    fn test_color_string_to_rsh_style_invalid_string() {
        let color_string = "invalid string".to_string();
        let style = color_string_to_rsh_style(color_string);
        assert_eq!(style, Style::default());
    }

    #[test]
    fn test_get_style_from_value() {
        // Test case 1: all values are valid
        let record = Record {
            cols: vec!["bg".to_string(), "fg".to_string(), "attr".to_string()],
            vals: vec![
                Value::string("red", Span::unknown()),
                Value::string("blue", Span::unknown()),
                Value::string("bold", Span::unknown()),
            ],
        };
        let expected_style = RshStyle {
            bg: Some("red".to_string()),
            fg: Some("blue".to_string()),
            attr: Some("bold".to_string()),
        };
        assert_eq!(get_style_from_value(&record), Some(expected_style));

        // Test case 2: no values are valid
        let record = Record {
            cols: vec!["invalid".to_string()],
            vals: vec![Value::nothing(Span::unknown())],
        };
        assert_eq!(get_style_from_value(&record), None);

        // Test case 3: some values are valid
        let record = Record {
            cols: vec!["bg".to_string(), "invalid".to_string()],
            vals: vec![
                Value::string("green", Span::unknown()),
                Value::nothing(Span::unknown()),
            ],
        };
        let expected_style = RshStyle {
            bg: Some("green".to_string()),
            fg: None,
            attr: None,
        };
        assert_eq!(get_style_from_value(&record), Some(expected_style));
    }

    #[test]
    fn test_parse_map_entry() {
        let mut hm = HashMap::new();
        let key = "test_key".to_owned();
        let value = Value::string("red", Span::unknown());
        parse_map_entry(&mut hm, &key, &value);
        assert_eq!(hm.get(&key), Some(&lookup_ansi_color_style("red")));
    }
}

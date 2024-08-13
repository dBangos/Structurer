// If its too slow
// Store the previous strings and jobs into state to not have to do them all over again if they are
// the same as the new
use egui::{text::LayoutJob, Color32, FontId, Stroke, TextFormat};

//Takes a string, returns a vector of strings and all their markup
pub fn markup_parse_string(input_string: String) -> Vec<(String, Vec<bool>)> {
    let mut result: Vec<(String, Vec<bool>)> = Vec::new();
    let mut italic: bool = false;
    let mut highlight: bool = false;
    let mut underline: bool = false;
    let mut heading1: bool = false;
    let mut heading2: bool = false;
    let chars: Vec<char> = input_string.chars().collect();
    if chars.len() < 3 {
        result.push((input_string, vec![false, false, false, false, false]));
        return result;
    }
    let mut current_string: String = String::new();
    let mut index = 0;
    while index < chars.len() - 3 {
        if chars[index] == '[' && chars[index + 1] == '!' && chars[index + 3] == ']' {
            if current_string.len() > 0 {
                result.push((
                    current_string.clone(),
                    vec![italic, highlight, underline, heading1, heading2],
                ));
                current_string = String::new();
            }
            if chars[index + 2] == 'l' {
                highlight = !highlight;
            } else if chars[index + 2] == 'i' {
                italic = !italic;
            } else if chars[index + 2] == 'u' {
                underline = !underline;
            } else if chars[index + 2] == 'h' {
                heading2 = !heading2;
            } else if chars[index + 2] == 'H' {
                heading1 = !heading1;
            }
            index += 4;
        } else {
            if index == chars.len() - 4 {
                current_string.push(chars[index]);
                current_string.push(chars[index + 1]);
                current_string.push(chars[index + 2]);
                current_string.push(chars[index + 3]);
            } else {
                current_string.push(chars[index]);
            }
            index += 1;
        }
    }
    if current_string.len() > 0 {
        result.push((
            current_string.clone(),
            vec![false, false, false, false, false],
        ));
    }
    return result;
}

//Takes a vector of strings and markups and returns a layoutjob
pub fn markup_construct_job(input: Vec<(String, Vec<bool>)>) -> egui::text::LayoutJob {
    let mut result: egui::text::LayoutJob = LayoutJob::default();
    for (in_str, in_bools) in input {
        let mut text_format: TextFormat = TextFormat::default();
        text_format.font_id = FontId::proportional(20.0);
        text_format.color = Color32::WHITE;
        if in_bools[0] {
            //italic
            text_format.italics = true;
        }
        if in_bools[1] {
            //highlight
            text_format.background = Color32::DARK_GRAY;
        }
        if in_bools[2] {
            //underline
            text_format.underline = Stroke::new(1.0, Color32::WHITE);
        }
        if in_bools[3] {
            //heading1
            text_format.font_id = FontId::proportional(30.0);
        }
        if in_bools[4] {
            //heading2
            text_format.font_id = FontId::proportional(25.0);
        }
        result.append(&in_str, 0.0, text_format);
    }
    return result;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(
            markup_parse_string("".to_string()),
            vec![("".to_string(), vec![false, false, false, false, false])]
        );
    }
    #[test]
    fn test_small() {
        assert_eq!(
            markup_parse_string("a".to_string()),
            vec![("a".to_string(), vec![false, false, false, false, false])]
        );
    }
    #[test]
    fn test_bold_leftover() {
        assert_eq!(
            markup_parse_string("[!b]bold[!b]not bold".to_string()),
            vec![
                ("bold".to_string(), vec![false, true, false, false, false]),
                (
                    "not bold".to_string(),
                    vec![false, false, false, false, false]
                )
            ]
        );
    }
    #[test]
    fn test_bold_exact() {
        assert_eq!(
            markup_parse_string("[!b]bold[!b]".to_string()),
            vec![("bold".to_string(), vec![false, true, false, false, false])]
        );
    }
    #[test]
    fn test_overlapping() {
        assert_eq!(
            markup_parse_string("[!b]bold[!i]bold and italic[!b]".to_string()),
            vec![
                ("bold".to_string(), vec![false, true, false, false, false]),
                (
                    "bold and italic".to_string(),
                    vec![true, true, false, false, false]
                )
            ]
        );
    }
    #[test]
    fn test_overlapping_more() {
        assert_eq!(
            markup_parse_string("nothing[!l]bold[!i]bold and italic[!l]".to_string()),
            vec![
                (
                    "nothing".to_string(),
                    vec![false, false, false, false, false]
                ),
                ("bold".to_string(), vec![false, true, false, false, false]),
                (
                    "bold and italic".to_string(),
                    vec![true, true, false, false, false]
                )
            ]
        );
    }
}

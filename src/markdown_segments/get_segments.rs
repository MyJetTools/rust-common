use std::collections::HashMap;

/// Finds segments inside text
/// in markup like [SEGMENT_NAME param1="xxx" param2="yyyy"]

#[derive(Debug)]
pub enum Segment<'s> {
    Text(&'s str),
    Segment(SegmentData<'s>),
}

#[derive(Debug)]
pub struct SegmentData<'s> {
    pub params: HashMap<&'s str, &'s str>,
    pub text: &'s str,
}

impl<'s> Segment<'s> {
    pub fn unwrap_as_text(&'s self) -> &'s str {
        match self {
            Segment::Text(text) => *text,
            Segment::Segment(data) => {
                panic!("Can not unwrap as text. Data is a segment: {:?}", data);
            }
        }
    }

    pub fn unwrap_as_segment(&'s self) -> &'s SegmentData<'s> {
        match self {
            Segment::Text(text) => {
                panic!("Can not unwrap as segment. Data is a text: {:?}", text);
            }
            Segment::Segment(data) => data,
        }
    }
}

pub fn get_markdown_segments<'s>(mut src: &'s str, segment_name: &'s str) -> Vec<Segment<'s>> {
    let mut result = vec![];

    let segment_to_search1 = format!("[{}]", segment_name);
    let segment_to_search2 = format!("[{} ", segment_name);

    while src.len() > 0 {
        let start_index1 = src.find(segment_to_search1.as_str());
        let start_index2 = src.find(segment_to_search2.as_str());

        let start_index = get_closest_index(start_index1, start_index2);

        let Some(start_index) = start_index else {
            result.push(Segment::Text(src));
            return result;
        };

        if start_index > 0 {
            result.push(Segment::Text(&src[..start_index]));

            src = &src[start_index..]
        };

        let end_marker = format!("[/{segment_name}]");

        let segment_text_start_index = src.find(']');

        let Some(mut segment_text_start_index) = segment_text_start_index else {
            return result;
        };

        segment_text_start_index += 1;

        let params = extract_params(&src[..segment_text_start_index]);

        let Some(end_index) = src.find(&end_marker) else {
            result.push(Segment::Segment(SegmentData {
                params,
                text: &src[segment_text_start_index..],
            }));
            return result;
        };

        result.push(Segment::Segment(SegmentData {
            params,
            text: &src[segment_text_start_index..end_index],
        }));

        src = &src[end_index..];

        let index = src.find(']').unwrap();

        src = &src[index + 1..];
    }

    result
}

fn extract_params<'s>(mut src: &'s str) -> HashMap<&'s str, &'s str> {
    let mut result = HashMap::new();

    let Some(param_start) = src.find(' ') else {
        return result;
    };

    src = &src[param_start + 1..src.len() - 1];

    while src.len() > 0 {
        src = src.trim_start();

        let Some(eq_index) = src.find('=') else {
            break;
        };

        let key = &src[..eq_index];

        src = &src[eq_index + 1..];

        let (value, env_of_value) = find_end_of_value(src);

        result.insert(key, value);

        src = &src[env_of_value..]
    }

    result
}

fn find_end_of_value<'s>(src: &'s str) -> (&'s str, usize) {
    let mut open_symbol = None;

    let mut escape = false;
    for (i, c) in src.chars().enumerate() {
        if i == 0 {
            if c == '"' || c == '\'' {
                open_symbol = Some(c);
            }

            continue;
        }

        if escape {
            escape = false;
            continue;
        }

        if c == '\\' {
            escape = true;
            continue;
        }

        match open_symbol {
            Some(open_symbol) => {
                if c == open_symbol {
                    return (&src[1..i], i + 1);
                }
            }
            None => {
                if c == ' ' {
                    return (&src[0..i], i);
                }
            }
        }
    }

    (src, src.len())
}

fn get_closest_index(index1: Option<usize>, index2: Option<usize>) -> Option<usize> {
    if let Some(index1) = index1 {
        if let Some(index2) = index2 {
            if index1 < index2 {
                return Some(index1);
            } else {
                return Some(index2);
            }
        }

        return Some(index1);
    }

    index2
}

#[cfg(test)]
mod test {

    #[test]
    fn test_no_segments() {
        let src = "This is my text";

        let mut result = super::get_markdown_segments(src, "PITCH");

        assert_eq!(result.len(), 1);

        let itm = result.remove(0);

        assert_eq!(src, itm.unwrap_as_text());
    }

    #[test]
    fn test_only_segment_not_finished() {
        let src = "[PITCH]PitchText";

        let mut result = super::get_markdown_segments(src, "PITCH");

        assert_eq!(result.len(), 1);

        let itm = result.remove(0);

        let result = itm.unwrap_as_segment();
        assert_eq!(result.params.len(), 0);
        assert_eq!("PitchText", result.text);
    }

    #[test]
    fn test_only_segment() {
        let src = "[PITCH]PitchText[/PITCH]";

        let mut result = super::get_markdown_segments(src, "PITCH");

        assert_eq!(result.len(), 1);

        let itm = result.remove(0);

        let result = itm.unwrap_as_segment();
        assert_eq!(result.params.len(), 0);
        assert_eq!("PitchText", result.text);
    }

    #[test]
    fn test_when_pitch_is_loaded_not_full() {
        let src = "Before pitch[PITCH id";

        let mut result = super::get_markdown_segments(src, "PITCH");

        assert_eq!(result.len(), 1);

        let itm = result.remove(0);

        let result = itm.unwrap_as_text();

        assert_eq!("Before pitch", result);
    }

    #[test]
    fn test_only_segment_two_parameters() {
        let src = r#"[PITCH param1="value 1" param2='value 2' param3=value3 param4=value4]PitchText[/PITCH]"#;

        let mut result = super::get_markdown_segments(src, "PITCH");

        assert_eq!(result.len(), 1);

        let itm = result.remove(0);

        let result = itm.unwrap_as_segment();
        assert_eq!(result.params.len(), 4);
        assert_eq!(result.params.get("param1").unwrap(), &"value 1");
        assert_eq!(result.params.get("param2").unwrap(), &"value 2");
        assert_eq!(result.params.get("param3").unwrap(), &"value3");
        assert_eq!(result.params.get("param4").unwrap(), &"value4");
        assert_eq!("PitchText", result.text);
    }

    #[test]
    fn test_two_segments_no_params() {
        let src = r#"BeforeText[PITCH id="15"]PitchText[/PITCH][PITCH]PitchText2[/PITCH]AfterText"#;

        let mut result = super::get_markdown_segments(src, "PITCH");

        assert_eq!(result.len(), 4);
        let itm = result.remove(0);
        assert_eq!("BeforeText", itm.unwrap_as_text());

        let itm = result.remove(0);
        let segment = itm.unwrap_as_segment();
        assert_eq!(segment.params.len(), 1);
        assert_eq!(segment.params.get("id").unwrap(), &"15");
        assert_eq!(segment.text, "PitchText");

        let itm = result.remove(0);
        let segment = itm.unwrap_as_segment();
        assert_eq!(segment.params.len(), 0);
        assert_eq!(segment.text, "PitchText2");

        let itm = result.remove(0);
        assert_eq!("AfterText", itm.unwrap_as_text());
    }
}

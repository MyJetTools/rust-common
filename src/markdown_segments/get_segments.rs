use std::collections::HashMap;

/// Finds segments inside text
/// in markup like [SEGMENT_NAME param1="xxx" param2="yyyy"]

pub struct Segment<'s> {
    pub id: &'s str,
    pub params: HashMap<String, String>,
    pub data: &'s str,
}

pub fn get_markdown_segments<'s>(src: &'s str, segment_name: &'s str) -> Vec<Segment<'s>> {
    let mut result = vec![];

    let mut segment_to_search = format!("[{}]", segment_name);

    loop {
        let mut start_index = src.find(segment_to_search.as_str());

        if start_index.is_none() {
            segment_to_search.pop();
            segment_to_search.push(' ');
            start_index = src.find(&segment_to_search)
        }

        let Some(start_index) = start_index else {
            result.push(Segment {
                id: "",
                params: HashMap::new(),
                data: src,
            });

            return result;
        };

        let src = if start_index > 0 {
            result.push(Segment {
                id: "",
                params: HashMap::new(),
                data: src,
            });

            &src[..start_index]
        } else {
            src
        };

        let end_marker = format!("[/{segment_name}]");

        let end_index = src.find(&end_marker);
    }

    result
}

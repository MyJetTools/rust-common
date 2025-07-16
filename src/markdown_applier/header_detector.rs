const MAX_SIZE: usize = 6;
pub struct HeaderDetector {
    value: Vec<u8>,
}

impl HeaderDetector {
    pub fn new() -> Self {
        Self {
            value: Vec::with_capacity(MAX_SIZE),
        }
    }

    fn push(&mut self, c: char) {
        if self.value.len() == MAX_SIZE {
            self.value.remove(0);
        }

        self.value.push(c as u8);
    }

    pub fn push_and_detect(&mut self, c: char) -> HeaderDetectionResult {
        match c {
            '#' => {
                self.push(c);
                return HeaderDetectionResult::InDetection;
            }

            '\n' => {
                if self.value.len() == 0 {
                    self.value.push(c as u8);
                }

                return HeaderDetectionResult::None;
            }

            ' ' => {
                let result = self.try_get_header();
                self.value.clear();
                if let Some(result) = result {
                    return HeaderDetectionResult::HasResult(result);
                }

                return HeaderDetectionResult::None;
            }

            _ => {
                self.value.clear();
                return HeaderDetectionResult::None;
            }
        }
    }

    fn try_get_header(&self) -> Option<u8> {
        if self.value == b"#" || self.value == b"\n#" {
            return Some(1);
        }

        if self.value == b"##" || self.value == b"\n##" {
            return Some(2);
        }

        if self.value == b"###" || self.value == b"\n###" {
            return Some(3);
        }

        if self.value == b"####" || self.value == b"\n####" {
            return Some(4);
        }

        None
    }
}

pub enum HeaderDetectionResult {
    None,
    InDetection,
    HasResult(u8),
}

use std::io::Error;

#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<String>,
}

//impl Default for Buffer {
//    fn default() -> Buffer {
//        Buffer {
//            lines: vec!["Hello Hlello".to_string(), "Hello Hlello".to_string()],
//        }
//    }
//}
//
//

impl Buffer {
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    pub fn load(file_name: &str) -> Result<Self, Error> {
        let contents = std::fs::read_to_string(file_name)?;
        let mut lines: Vec<String> = Vec::new();

        for line in contents.lines() {
            lines.push(String::from(line));
        }

        Ok(Self { lines })
    }
}

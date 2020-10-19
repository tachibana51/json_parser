pub struct Parser {
    pub input: String,
    pos: usize,
}

pub enum JSONValue {
    JString(String),
    Number(i64),
    Object(Vec<JSONObject>),
    Array(Vec<JSONValue>),
    Bool(bool),
    Null,
}

enum JSONToken {}

pub struct JSONObject {
    pub key: String,
    pub value: JSONValue,
}

pub struct JSON {
    pub value: Vec<JSONObject>,
}

impl Parser {
    pub fn run(&mut self) {
        unimplemented!()
    }

    fn count_whitespaces_from_pos(&self) -> usize {
        let mut count = 0;
        for c in self.input[self.pos..].chars() {
            if !is_white_space(c) {
                return count;
            }
            count += 1;
        }
        count
    }

    fn char_at(&self, idx: usize) -> char {
        self.input[idx..].chars().next().unwrap()
    }

    //TODO: Error Handling
    fn parse_string(&mut self) -> JSONValue {
        let mut parsed_string = String::new();
        let mut count = 0;
        while self.pos + count < self.input.chars().count() {
            let c = match self.char_at(self.pos + count) {
                '"' => {
                    if count != 0 {
                        break;
                    }
                    count += 1;
                    continue;
                }
                '\\' => match self.char_at(self.pos + count + 1) {
                    'n' => {
                        count += 1;
                        '\n'
                    }
                    'r' => {
                        count += 1;
                        '\r'
                    }
                    't' => {
                        count += 1;
                        '\t'
                    }
                    '"' => {
                        count += 1;
                        '"'
                    }
                    '\\' => {
                        count += 1;
                        '\\'
                    }
                    _ => {
                        count += 1;
                        continue;
                    }
                },
                other => other,
            };
            parsed_string.push(c);
            count += 1;
        }
        self.pos += count + 1;
        JSONValue::JString(parsed_string)
    }

    fn parse_number(&mut self) -> JSONValue {
        let mut parsed_string = String::new();
        let mut count = 0;
        while self.pos + count < self.input.chars().count() {
            let c = match self.char_at(self.pos + count) {
                '-' => {
                    count += 1;
                    if count != 1 {
                        break;
                    }
                    continue;
                }
                '0'..='9' => self.char_at(self.pos + count),
                _ => {
                    break;
                }
            };
            parsed_string.push(c);
            count += 1;
        }
        self.pos += count + 1;

        JSONValue::Number(parsed_string.parse::<i64>().unwrap())
    }

    fn is_bool(&mut self) -> bool {
        let mut count = 0;
        if self.input[self.pos..]
    } 

    fn parse_value() -> JSONValue {
        unimplemented!()
    }

    fn parse_array() -> JSONValue {
        unimplemented!()
    }

    fn parse_object() -> JSONValue {
        unimplemented!()
    }
}

fn is_white_space(c: char) -> bool {
    c == ' ' || c == '\n' || c == '\t' || c == '\r'
}

pub fn new_parser(input: String) -> Parser {
    Parser {
        input: input,
        pos: 0,
    }
}

pub fn draft() {
    //{“taro”: 12,“jiro”: {“saburo”: “shiro”},“goro”: true}
    let _first = JSON {
        value: vec![
            JSONObject {
                key: "taro".to_string(),
                value: JSONValue::Number(12),
            },
            JSONObject {
                key: "jiro".to_string(),
                value: JSONValue::Object(vec![JSONObject {
                    key: "saburo".to_string(),
                    value: JSONValue::JString("shiro".to_string()),
                }]),
            },
            JSONObject {
                key: "goro".to_string(),
                value: JSONValue::Bool(true),
            },
        ],
    };

    //{“taro”: []}
    let _second = JSON {
        value: vec![JSONObject {
            key: "taro".to_string(),
            value: JSONValue::Array(vec![]),
        }],
    };
    //let mut p = new_parser(r#"{"taro": 12,"jiro": {"saburo": "shiro"},"goro": true}"#.to_owned());
    let mut p = new_parser(r#"12,"jiro": {"saburo": "shiro"},"goro": true}"#.to_owned());
}

pub struct Parser {
    pub input: String,
    pos: usize,
}

#[derive(Debug)]
pub enum JSONValue {
    JString(String),
    Number(i64),
    Object(Vec<JSONObject>),
    Array(Vec<JSONValue>),
    Bool(bool),
    Null,
}

enum JSONToken {}

#[derive(Debug)]
pub struct JSONObject {
    pub key: String,
    pub value: JSONValue,
}

#[derive(Debug)]
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

    fn prefetch_string(&self, length: usize) -> &str {
        &self.input[self.pos..length]
    }

    fn get_keyword(&mut self) -> JSONValue {
        match self.prefetch_string(4) {
            "null" => {
                self.pos += 4;
                JSONValue::Null
            }
            "true" => {
                self.pos += 4;
                JSONValue::Bool(true)
            }
            _ => match self.prefetch_string(5) {
                "false" => {
                    self.pos += 5;
                    JSONValue::Bool(false)
                }
                _ => panic!("unexpected keyword"),
            },
        }
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
                '+' | '-' => {
                    if count != 0 {
                        break;
                    }
                    self.char_at(self.pos + count)
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

    fn parse_value(&mut self) -> JSONValue {
        match self.char_at(self.pos) {
            '"' => self.parse_string(),
            '+' | '-' | '0'..='9' => self.parse_number(),
            _ => self.get_keyword(),
        }
    }

    fn parse_array(&mut self) -> JSONValue {
        let mut parsed_array: Vec<JSONValue> = Vec::new();
        loop {
            let v = match self.char_at(self.pos) {
                ']' => {
                    break;
                }
                ',' => {
                    self.pos += 1;
                    continue;
                }
                other => {
                    if is_white_space(self.char_at(self.pos)){
                        self.pos += 1;
                        continue;
                    }
                    self.parse_value()
                },
            };
            parsed_array.push(v);
        }
        self.pos += 1;
        JSONValue::Array(parsed_array)
    }

    fn parse_object(&mut self) -> JSONObject {
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
    println!("{:#?}", p.parse_number());
}

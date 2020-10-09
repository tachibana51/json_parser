pub struct Parser {
    pub input: String,
    pos: u64,
}

pub enum JSONValue {
    JString(String),
    Number(i64),
    Object(Vec<JSONObject>),
    Array(Vec<JSONValue>),
    Bool(bool),
    Null,
}

pub struct JSONObject {
    pub key: String,
    pub value: JSONValue,
}

pub struct JSON {
    pub value: Vec<JSONObject>,
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
}

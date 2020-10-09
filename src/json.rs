pub struct Parser {
    pub input: String,
    pos: u64,
}

pub enum JsonValue {
    JString(String),
    Number(i64),
    Object(Vec<JsonObject>),
    Array(Vec<JsonValue>),
    Bool(bool),
    Null,
}

pub struct JsonObject {
    pub key: String,
    pub value: JsonValue,
}

pub struct Json {
    pub value: Vec<JsonObject>,
}

pub fn draft() {
    //{“taro”: 12,“jiro”: {“saburo”: “shiro”},“goro”: true}
    let _first = Json {
        value: vec![
            JsonObject {
                key: "taro".to_string(),
                value: JsonValue::Number(12),
            },
            JsonObject {
                key: "jiro".to_string(),
                value: JsonValue::Object(vec![JsonObject {
                    key: "saburo".to_string(),
                    value: JsonValue::JString("shiro".to_string()),
                }]),
            },
            JsonObject {
                key: "goro".to_string(),
                value: JsonValue::Bool(true),
            },
        ],
    };

    //{“taro”: []}
    let _second = Json {
        value: vec![JsonObject {
            key: "taro".to_string(),
            value: JsonValue::Array(vec![]),
        }],
    };
}

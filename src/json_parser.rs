use json_parse::{JSONString, JSONTYPE};

/// parse json string to map
/// use case: to test a json parse function
///
/// pub fn test_parse_json(){
///  let json = r#"
///  {
///      "name":"json",
///      "age":18,
///      "is_student":true,
///      "score":[1,2,3,4,5],
///      "info":{
///          "address":"beijing",
///          "phone":"123456789"
///      }
///  }"#;
///  let mut json_string = JSONString::new(json.to_string());
///  let json: JSONTYPE = json_string.parse();
///  println!("{:?}",json);
///
///  let json_array_str = r#"
///  [
///  {
///      "name":"json",
///      "age":18,
///      "is_student":true,
///      "score":[1,2,3,4,5],
///      "info":{
///          "address":"beijing",
///          "phone":"123456789"
///      }
///  },
///  {
///      "name":"json",
///      "age":18,
///      "is_student":true,
///      "score":[1,2,3,4,5],
///      "info":{
///          "address":"beijing",
///          "phone":"123456789"
///      }
///  }
///  ]"#;
///  let mut json_string = JSONString::new(json_array_str.to_string());
///  let json: JSONTYPE = json_string.parse();
///  println!("{:?}",json);
///  }
///
///
///
pub mod json_parse {
    use core::panic;
    use std::{char, collections::HashMap};

    #[derive(Debug)]
    pub enum JSONTYPE {
        Null,
        Bool(bool),
        String(String),
        Number(f64),
        Object(HashMap<String, JSONTYPE>),
        Array(Vec<JSONTYPE>),
    }

    pub struct JSONString {
        data: String,
        index: usize,
    }

    impl JSONString {
        pub fn new(data: String) -> JSONString {
            JSONString { data, index: 0 }
        }

        fn next(&mut self) -> Option<char> {
            if self.data.len() > self.index {
                let c = self.data.chars().nth(self.index);
                self.index += 1;
                c
            }else{
                None
            }
        }

        fn peek(&self)->Option<char>{
            if self.data.len()>self.index{
                let ch = self.data.chars().nth(self.index);
                ch
            }else{
                None
            }
        }

        fn skip_whitespace(&mut self){
            while self.data.chars().nth(self.index).unwrap().is_whitespace(){
                self.index+=1;
            }
        }

        fn parse_null(&mut self)->JSONTYPE{
            if self.data.len()>=self.index+4{
                let null = self.data.chars().skip(self.index).take(4).collect::<String>();
                if null == "null"{
                    self.index+=4;
                    return JSONTYPE::Null;
                }
            }
            panic!("yuor json start with n but not null");
        }
        fn parse_bool(&mut self)->JSONTYPE{
            let ch = self.peek();
            if ch.unwrap() == 't'{
                if self.data.len()>= self.index+4{
                    let true_str = self.data.chars().skip(self.index).take(4).collect::<String>();
                    if true_str == "true"{
                        self.index+=4;
                        return JSONTYPE::Bool(true);
                    }
                }
            }else if ch.unwrap() == 'f'{
                if self.data.len() >= self.index+5{
                    let false_str = self.data.chars().skip(self.index).take(5).collect::<String>();
                    if false_str == "false"{
                        self.index+=5;
                        return JSONTYPE::Bool(false);
                    }
                }
            }
            panic!("your json start with t or f but not true or false");
        }

        fn parse_string(&mut self)->JSONTYPE{
            let mut result = String::new();
            let mut escape = false;
            let mut unicode = false;
            let mut unicode_hex = 0;
            let mut unicode_count = 0;
            loop{
                let ch = self.next();
                match ch{
                    Some('"')=>{
                        if !escape{
                            return JSONTYPE::String(result);
                        }else{
                            result.push('"');
                        }
                    },
                    Some('\\')=>{
                        if escape{
                            result.push('\\');
                            escape = false;
                        }else{
                            escape = true;
                        }
                    },
                    Some('b')=>{
                        if !escape{
                            result.push('b');
                        }else{
                            result.push('\u{0008}');
                            escape = false;
                        }
                    },
                    Some('f')=>{
                        if !escape{
                            result.push('f');
                        }else{
                            result.push('\u{000c}');
                            escape = false;
                        }
                    },
                    Some('n')=>{
                        if escape{
                            result.push('\n');
                            escape = false;
                        }else{
                            result.push('n');
                        }
                    },
                    Some('r')=>{
                        if escape{
                            result.push('\r');
                            escape = false;
                        }else{
                            result.push('r');
                        }
                    },
                    Some('t')=>{
                        if escape{
                            result.push('\t');
                            escape = false;
                        }else{
                            result.push('t');
                        }
                    },
                    Some('u')=>{
                        if escape{
                            if unicode{
                                panic!("parse string error");
                            }
                            unicode = true;
                            unicode_count = 0;
                        }else{
                            result.push('u');
                        }
                    },
                    Some(c)=>{
                        if unicode{
                            if c.is_digit(16){
                                unicode_hex = unicode_hex*16 + c.to_digit(16).unwrap();
                                unicode_count+=1;
                                if unicode_count == 4{
                                    result.push(char::from_u32(unicode_hex).unwrap());
                                    unicode = false;
                                }
                            }else{
                                panic!("parse string error");
                            }
                        }else{
                            result.push(c);
                        }
                    },
                    None=>{
                        panic!("parse string error");
                    }
                }
            }
        }

        pub fn parse_object(&mut self)->JSONTYPE{
            let mut object = HashMap::new();
            loop{
                self.skip_whitespace();
                let ch = self.next();
                match ch {
                    Some('}')=>{
                        return JSONTYPE::Object(object);
                    },
                    Some(',')=>{
                        continue;
                    },
                    Some(c)=>{
                        let key = self.parse_string();
                        self.skip_whitespace();
                        let ch = self.next();
                        if ch != Some(':'){
                            panic!("parse object error");
                        }
                        let value = self.parse_value();
                        object.insert(
                            match key{
                                JSONTYPE::String(s)=>s,
                                _=>panic!("parse object error")
                            }, value);
                    },
                    None=>{
                        panic!("parse object error");
                    }
                }
            }
        }

        fn parse_value(&mut self)->JSONTYPE{
            self.skip_whitespace();
            let ch = self.peek();
            match ch {
                Some('n')=>{
                    self.parse_null()
                },
                Some('t')|Some('f')=>{
                    self.parse_bool()
                },
                Some('"')=>{
                    self.next();
                    self.parse_string()
                },
                Some('{')=>{
                    self.next();
                    self.parse_object()
                },
                Some('[')=>{
                    self.next();
                    self.parse_array()
                },
                Some(c)=>{
                    if c.is_digit(10){
                        self.parse_number()
                    }else{
                        panic!("parse value error");
                    }
                },
                None=>{
                    panic!("parse value error");
                }
            }
        }

        fn parse_number(&mut self)->JSONTYPE{
            let mut number = String::new();
            self.skip_whitespace();
            loop{
                let ch = self.peek();
                match ch{
                    Some(c)=>{
                        if c.is_digit(10) || c == '.' || c == 'e' || c == 'E' || c == '+' || c == '-'{
                            number.push(c);
                            self.next();
                        }else{
                            break;
                        }
                    },
                    None=>{
                        break;
                    }
                }
            }
            let num = number.parse::<f64>().unwrap();
            JSONTYPE::Number(num)
        }

        fn parse_array(&mut self)->JSONTYPE{
            let mut array = Vec::new();
            loop{
                self.skip_whitespace();
                let ch = self.peek();
                match ch{
                    Some(']')=>{
                        self.next();
                        return JSONTYPE::Array(array);
                    },
                    Some(',')=>{
                        self.next();
                        continue;
                    },
                    Some(_)=>{
                        let value = self.parse_value();
                        array.push(value);
                    },
                    None=>{
                        panic!("parse array error");
                    }
                }
            }
        }
        // parse jsonobject or jsonarray
        pub fn parse(&mut self)->JSONTYPE{
            self.skip_whitespace();
            // first char like [ or {
            let ch = self.next();
            match ch {
                Some('{')=>{
                    return self.parse_object();
                },
                Some('[')=>{
                    return self.parse_array();
                },
                Some(_)=>{
                    panic!("your json no start with [ or '{{");
                },
                None=>{
                    panic!("your json no start with [ or '{{");
                }
            }
        }
    }


}

// test the result or parse
pub fn test_parse_json(){
    let json = r#"
    {
        "name":"json",
        "age":18,
        "is_student":true,
        "score":[1,2,3,4,5],
        "info":{
            "address":"beijing",
            "phone":"123456789"
        }
    }"#;
    let mut json_string = JSONString::new(json.to_string());
    let json: JSONTYPE = json_string.parse();
    println!("{:?}",json);

    let json_array_str = r#"
    [
    {
        "name":"json",
        "age":18,
        "is_student":true,
        "score":[1,2,3,4,5],
        "info":{
            "address":"beijing",
            "phone":"123456789"
        }
    },
    {
        "name":"json",
        "age":18,
        "is_student":true,
        "score":[1,2,3,4,5],
        "info":{
            "address":"beijing",
            "phone":"123456789"
        }
    }
    ]"#;
    let mut json_string = JSONString::new(json_array_str.to_string());
    let json: JSONTYPE = json_string.parse();
    println!("{:?}",json);
}
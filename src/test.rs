use uuid::Uuid;
use serde::{Serialize, Deserialize, };

type SPJson = serde_json::Value;
type SPJsonError = serde_json::Error;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
struct SPAttributes {
    attr: SPJson
}

impl SPAttributes {
    fn test(&self) {
        println!("{:?}", &self.attr)
    }

    pub fn make<T>(value: T) -> Result<SPAttributes, SPJsonError> where T: Serialize {
        serde_json::to_value(value).map(|v| SPAttributes{attr: v})
    }

}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum Predicate {
    AND(Vec<Predicate>),
    OR(Vec<Predicate>),
    Not(Box<Predicate>),
    TRUE,
    FALSE,
    EQ(SPValue, SPValue),
    NEQ(SPValue, SPValue),
    INDOMAIN(SPValue, Vec<SPValue>)
}


#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum SPValue {
    Bool(bool),
    //byte(u8), deprecated
    //char(char), deprecated
    Float32(f32), 
    Float64(f64),
    Int8(i8), 
    Uint8(u8),
    Int16(i16), 
    Uint16(u16),
    Int32(i32), 
    Uint32(u32),
    Int64(i64), 
    Uint64(u64),
    String(String),
    Time(u32),
    Duration(u32),
    ID(Uuid), 
    Array(Vec<SPValue>),
    Map(std::collections::HashMap<String, SPValue>)
}

struct SPObject {
    id: Uuid,
    name: String,
    attributes: SPAttributes,
}


trait IDAble {
    fn id(&self) -> Uuid;
    fn name(&self) -> String;
    fn attributes(&self) -> SPAttributes; // use serde Value enum
}

#[derive(Debug, PartialEq)]
struct SPVariable {
    id: Uuid,
    name: String,
    attributes: SPAttributes,
    domain: Vec<SPValue>,
    init: SPValue
}




#[cfg(test)]
mod mytest {
    use super::Predicate::*;
    use super::*;

    #[test]
    fn testing() {
        let test = AND(vec!(OR(vec!(TRUE))));
        let kalle = SPAttributes::make(test).unwrap();

        kalle.test()
    }
}
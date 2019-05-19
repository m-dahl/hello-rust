use uuid::Uuid;
use serde::{Serialize, Deserialize, };
use std::collections::HashMap;



#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
enum Predicate {
    AND(Vec<Predicate>),
    OR(Vec<Predicate>),
    Not(Box<Predicate>),
    TRUE,
    FALSE,
    EQ(SPValue, SPValue),  // use SPValue::ID to fetch the value from the state
    NEQ(SPValue, SPValue),
    INDOMAIN(SPValue, Vec<SPValue>)
}


#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
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
    ID(Uuid),     // use to also find the value in a state of variable with id
    Array(Vec<SPValue>),
    Map(HashMap<String, SPValue>)
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
struct SPState {
    s: HashMap<String, SPValue>
}

//#[derive(Debug, PartialEq)]
// Was hard to define something to store genereal functions
// struct StateTransformation {
//     tf: Fn(SPState) -> SPState,  // will be used by the runner
//     to_action: Fn(SPState -> Vec<Action>)  // will be used for converting to a formal model
// }

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
struct Action {
    var: Uuid,
    value: Compute
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
/// Used in actions to compute a new SPValue
enum Compute {
    Value(SPValue),
    Get(Uuid),
    TakeNext(Uuid, Vec<SPValue>), 
    TakeBefore(Uuid, Vec<SPValue>),
    Add(Box<Compute>, Box<Compute>),
    Sub(Box<Compute>, Box<Compute>),
    Join(Box<Compute>, Box<Compute>),
}


#[cfg(test)]
mod testing_predicates {
    use super::Predicate::*;
    use super::*;

    #[test]
    fn testing() {
        let test = AND(vec!(OR(vec!(TRUE))));
        let kalle = SPAttributes::make(test).unwrap();

        kalle.test();

        let id = uuid::Uuid::new_v4();

        let a = Action{var: id, value: Compute::Get(id)};

        println!("{:?}", a)


    }
}





///**************
///  



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
    sp: SPObject,
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
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct Work {
    pub id: i32,
    pub work_code: String,
    pub add_up_to: i32,
    pub done: bool,
}

// implement the "Display" trait for the struct "Work"
// This will enable us to use a simple {} when using println!
impl std::fmt::Display for Work {
    fn fmt(&self, f : &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\n---\nWORK:\n id: {}\n work_code: {}\n add_up_to: {}\n done: {}\n---",
            self.id, self.work_code, self.add_up_to, self.done,
        )
    }
}

fn main() {

    let work = Work {
        id: 1,
        work_code: "foo".to_string(),
        add_up_to: 100,
        done: false,
    };

    // Convert/serialize the work variable (Work struct) to json string
    // the below function serializes a given data structure to a string of Json
    let work_serlzd_json = serde_json::to_string(&work).unwrap();
    println!("Serialized work is {}", work_serlzd_json);

    //Deserialize or convert the above Json string to a work struct data structure
    // using serde
    // use deserializer from_str() function. This takes a string slice so convert
    // json string to string slice using as_str() function
    let work_from_json: Work = serde_json::from_str(work_serlzd_json.as_str()).unwrap();
    println!("Deserialized: {:?}", work_from_json);

    //We can check that work_from_json is indeed a Work instance by using the
    // Rust asset_eq!() function. If the two are not equivalent, this function
    // would return an error
    assert_eq!(work, work_from_json);

    let work_from_json2: Work = serde_json::from_str(work_serlzd_json.as_str()).unwrap();
    // print using the custom Display trait implemented for Work struct
    // note that we do not use the default {:?} debug display anymore
    println!("Deserialized V2: {}", work_from_json2);



}
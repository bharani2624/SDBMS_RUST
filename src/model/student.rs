use serde::{Serialize,Deserialize};
// use uuid::Uuid;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Student{
    #[serde(rename = "_key")]
    pub id:String,
    pub name:String,
    pub age:u8,//0 to 255
    pub subject:String
}
impl Student {
    pub fn new(name:String,age:u8,subject:String,id:String)-> Self{
        Self{
            name,
            age,
            subject,
            id
        }
    }
}
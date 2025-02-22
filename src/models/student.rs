use serde::{Serialize,Deserialize};
use uuid::Uuid;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Student{
    pub id:String,
    pub name:String,
    pub age:u8,//0 to 255
    pub subject:String
}
impl Student {
    pub fn new(name:String,age:u8,subject:String)-> Self{
        Self{
            name,
            age,
            subject,
            id:Uuid::new_v4().to_string()
        }
    }
}
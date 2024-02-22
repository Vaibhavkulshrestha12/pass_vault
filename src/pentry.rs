 use serde::{Deserialize,Serialize};
 use std::fmt::format;
use std::fs::read;
use std::fs::File;
 use std::fs::OpenOptions;
 use std::io;
 use std::io::BufRead;
 use std::io::Read;
use std::io::Write;

#[derive(Debug, Serialize,Deserialize)]
 pub struct ServiceInfo{
    pub service: String,
    pub username: String,
    pub password: String,
 }
 impl ServiceInfo{
 pub fn new (service: String, username:String,password: String)-> Self{
     ServiceInfo {
        service,
        username,
        password,
     }
 } 
  pub fn from_json(json_string: &str) ->Result<Self, serde_json::Error >{
    serde_json::from_str(json_string)
  }
   #[allow(dead_code)]
   pub fn from_user_input() -> Self{
    println!("Enter the password Entery: ");
    let mut service =String::new();
    io::stdin()
     .read_line(&mut service)
     .expect("Failed to read line ");

    println!("Enter username : ");  
    let mut username =String::new();
    io::stdin()
     .read_line(&mut username)
     .expect("Failed to read line ");


     println!("Enter password: ");  
     let mut password =String::new();
     io::stdin()
      .read_line(&mut password)
      .expect("Failed to read line ");
    ServiceInfo::new(service.trim().to_string(), username.trim().to_string(), password.trim().to_string())
   }
   pub fn to_json(&self) ->String{
    serde_json::to_string(&self).expect("Failed to serialize to JSON")

   }

   pub fn write_to_file(&self){
    let json_output =format("{}\n", self.to_json());


    match OpenOptions::new()
      .create(true)
      .append(true)
      .open("passwords.json")
      {
        Ok (mut file ) => {
            file.write_all(json_output.as_bytes()){
                 eprintln!("Error writing to file : {}",e);
            } else {
                println!("successfully wrote to passwords.json")
            }
        }
         Err(e) => eprintln!("Error opening file :{}",e);





      }
   }



 }
 pub fn read_password_from_file() -> Result<Vec<ServiceInfo>,io::Error{
    let file = File::open("password.json")?;
    let reader =io::BufReader::new(file);
    let mut services = Vec::new();
    for line in reader.line(){
        if let Ok(json_string)={
            if
        }
    }
 }
 pub fn prompt(prompt &str) -> String{
    print!("{}",prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
  
    io::stdin().read_line(&mut input). unwrap();
    input.trim().to_string()
 }
 
 

 
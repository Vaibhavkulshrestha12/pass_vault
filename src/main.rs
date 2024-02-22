mod  pentry;

use crate::pentry::prompt;
use crate::pentry::read_password_from_file;

fn clr(){
    print!("{}[2J",27 as char);

}
fn main(){
    clr();
    let ascii =r#"
    #    #                                                                           
    #   #    ##   #    #  ####  #    # # #    #    #    #   ##   #    # #      ##### 
    #  #    #  #  #    # #      #    # # #   #     #    #  #  #  #    # #        #   
    ###    #    # #    #  ####  ###### # ####      #    # #    # #    # #        #   
    #  #   ###### #    #      # #    # # #  #      #    # ###### #    # #        #   
    #   #  #    # #    # #    # #    # # #   #      #  #  #    # #    # #        #   
    #    # #    #  ####   ####  #    # # #    #      ##   #    #  ####  ######   # 
     "#;
     println!("{ascii}");
     loop {
         println!("Password manager menu:");
         println!("2. Add Entery:");
         println!("3.List Entries:");
         println!("4. Quit:");
         let mut choice =String::new();
         std::io::stdin().read_line(&mut choice).unwrap();
         match choice.trim() {
            "1"=> {
             clr();
             let entry = Service::info (
                
             )
            }
             "2" => {
              clr();
              let services = read_password_from_file ().unwrap_or_else(|err|{
                eprintln!("Error reading passwords :{}",err);
                Vec::new()
            });
             for item in &services {
                println!(
                    "Service ={}
                    -Username : {}
                    -Password : {}",
                    item.service,item.username,item.password
                );
             }
              
             }
             "3" => {
                clr();
                let services = read_password_from_file ().unwrap_or_else(|err|{
                  eprintln!("Error reading passwords :{}",err);
                  Vec::new()
              });
                let search =prompt("Search :");
                for item in &services{
                    if item.service.as_str() ==search.as_str(){
                        println!(
                            "Service ={}
                            -Username : {}
                            -Password : {}",
                            item.service,item.username,item.password
                        )
                    }
                }
             }
             "4" => {
                clr();
                println!("Goodbye!");
                break;
                
             }
             _=> println!("Invalid choice."),


         }
         println!("\n\n");
     }

}


use std::fs;
use std::path::Path;


pub fn run(path_str:&str){
    let path = Path::new(path_str);

    if !path.exists(){
        eprintln!(" Path does not exist:{}", path.display());
    }

    if !path.is_dir(){
        eprintln!("Provided path is not a directory:{}", path.display());
    }
    
    listallfiles(path);
}

fn listallfiles(path:&Path){
    match fs::read_dir(path){
        
        Ok(entries) => {
            
            println!("Listing contents of: {}", path.display());
            
            for entry_result in entries {
                
                match entry_result{
                    
                    Ok(entry) => {
                        
                        let file_type = entry.file_type();
                        let metadata = entry.metadata();
                        let file_name = entry.file_name();
                        let name = file_name.to_string_lossy();

                        match(file_type, metadata){
                            
                            (Ok(ft), Ok(md)) => {
                                
                                match ft.is_dir() {
                                    
                                    true => { 
                                        println!("Dir {}",name); 
                                        let new_path = entry.path();
                                        listallfiles(&new_path);
                                    },

                                    false => {
                                        let size = md.len();
                                        println!("[File] {:20} - {} bytes", name, size);
                                    }
                                }
                            }

                            _ => println!("[Unknown] {}", name),
                        }
                        
                    }

                    Err(err) => {
                        eprintln!("Could not read entry: {}", err);
                    }
                }
            }
        }

        Err(e) => eprintln!("Failed to read directory:{}", e),
    }

}

use serde_json::Value as JsonValue;
use serde_yaml::Value as YamlValue;
use std::fs;
use std::path::Path;

pub fn run(file_path: &str){
    let path = Path::new(file_path);

    if !path.exists(){
        eprintln!(" file does not exists:{}", file_path);
        return;
    }

    let content = match fs::read_to_string(path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to read file:{}", e);
            return;
        }
    };

    if file_path.ends_with(".json"){
        match parse_and_print_json(&content){
            Ok(_) => {}
            Err(e) => eprintln!("YAML Error:{}", e),
        }
    } else if file_path.ends_with(".yaml") || file_path.ends_with(".yml") {
        match parse_and_print_yaml(&content){
            Ok(_) => {}
            Err(e) => eprintln!("JSON Error:{}", e),
        }
    }else {
        eprintln!("Unsupported file type. Use .json or .yaml or .yml");
    }

}

fn parse_and_print_json(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let parsed: JsonValue = serde_json::from_str(input)?;
    println!("Parsed JSON:\n{}", serde_json::to_string_pretty(&parsed)?);
    Ok(())
}

fn parse_and_print_yaml(input: &str) -> Result<(), Box<dyn std::error::Error>> {
        let parsed: YamlValue = serde_yaml::from_str(input)?;
         println!("Parsed Yaml:\n{}", serde_yaml::to_string(&parsed)?);
         Ok(())
}

use std::vec::{Vec};
use std::string::{String};




#[derive(Debug)]
#[derive(PartialEq)]
pub enum FlagType {
    Server,
    Port,
}

#[derive(Debug)]
pub struct Flag{
    pub flag_type: FlagType,
    pub raw_value: String,
}

impl Flag{
    pub fn new(strarg: String) -> Flag{
        
        // println!("Str ARG: {}",strarg);
        if !strarg.contains("-") {
            panic!("Flag not formated properly: {}",strarg);
        }
        if strarg.contains("=") {
            let split : Vec<String> = strarg.split("=").map(|strref| strref.to_string()).collect();
            let flag_string  = split.get(0).unwrap().to_string();
        //println!("{}",flag_string);
            let raw_value = split.get(1).unwrap().to_string();
            return Flag{
                flag_type: Flag::flag_type_from_string(flag_string).unwrap(),
                raw_value: raw_value,
            };
        }
        Flag{
            flag_type: Flag::flag_type_from_string(strarg).unwrap(),
            raw_value: "".to_string(),
        }
        
        
        
    }
    fn flag_type_from_string(str_flag: String) -> Result<FlagType,String>{
        //let str_flag = str_flag.unwrap();
        match str_flag.to_ascii_lowercase().as_ref() {
            "-server" => Ok(FlagType::Server),
            "-port" => Ok(FlagType::Port),
            // "-c" => Ok(FlagType::Count),
            _=> Err(format!("Invalid Flag Name: {}",str_flag)),
            
            
        }
    }
    
    
}

pub fn get_flags() -> Vec::<Flag>{
    let mut flag_vec = Vec::new();
    let mut is_first = true;
    for argument in std::env::args() {
        if is_first { is_first = false; continue;}
        flag_vec.push(Flag::new(argument));
    }
    flag_vec
}

pub fn arg_count() -> i32 {
    let mut arg_count = 0;
    std::env::args().for_each(|_| {
        arg_count+=1;
    });
    arg_count
}


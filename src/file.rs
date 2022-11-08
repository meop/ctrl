use std::env::current_exe;
use std::error::Error;

use which::which;

pub(crate) fn get_cur_path_str() -> Result<String, Box<dyn Error>> {
    Ok(current_exe()?.to_str().unwrap().to_string())
}

pub(crate) fn exists_in_path(program: &str) -> bool {
    match which(program) {
        Ok(t) => match t.to_str() {
            Some(_s) => true,
            None => false,
        },
        Err(_e) => false,
    }
}

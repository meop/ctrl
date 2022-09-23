use which::which;

pub(crate) fn exists_in_path(prog: &str) -> bool {
    match which(prog) {
        Ok(t) => match t.to_str() {
            Some(_s) => true,
            None => false,
        },
        Err(_e) => false
    }
}

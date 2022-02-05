use std::process::ExitStatus;

pub struct Pm;

impl super::PackageManager for Pm {
    fn add(&self, list: &Vec<String>, force: &bool) -> Result<ExitStatus, std::io::Error> {
        todo!()
    }

    fn find(&self, pattern: &String) -> Result<ExitStatus, std::io::Error> {
        todo!()
    }

    fn list(&self, pattern: &Option<String>) -> Result<ExitStatus, std::io::Error> {
        todo!()
    }

    fn out(&self) -> Result<ExitStatus, std::io::Error> {
        todo!()
    }

    fn rem(&self, list: &Vec<String>, force: &bool) -> Result<ExitStatus, std::io::Error> {
        todo!()
    }

    fn up(&self, list: &Vec<String>) -> Result<ExitStatus, std::io::Error> {
        todo!()
    }
}

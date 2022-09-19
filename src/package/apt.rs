use std::process::ExitStatus;

impl super::PackageManagerOps for super::PackageManager {
    fn add(&self, list: &Vec<String>, force: &bool) -> Result<ExitStatus, std::io::Error> {
        todo!()
    }

    fn find(&self, pattern: &String) -> Result<ExitStatus, std::io::Error> {
        todo!()
    }

    fn list(&self, pattern: &Option<String>) -> Result<ExitStatus, std::io::Error> {
        todo!()
    }

    fn outdated(&self) -> Result<ExitStatus, std::io::Error> {
        todo!()
    }

    fn remove(&self, list: &Vec<String>, force: &bool) -> Result<ExitStatus, std::io::Error> {
        todo!()
    }

    fn upgrade(&self, list: &Vec<String>) -> Result<ExitStatus, std::io::Error> {
        todo!()
    }
}

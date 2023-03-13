use crate::data::RequestJson;
use crate::errors::FileLoadError;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub fn get_json_from_file(path: &Path) -> Result<RequestJson, FileLoadError> {
    let f: Result<File, std::io::Error> = File::open(path);
    if f.is_err() {
        let path_str = match path.to_str() {
            Some(p) => p.to_owned(),
            None => "".to_owned(),
        };
        return Err(FileLoadError::FileNotFound(path_str));
    }
    let reader = BufReader::new(f.unwrap());
    let resp: Result<RequestJson, serde_json::Error> = serde_json::from_reader(reader);
    match resp {
        Ok(x) => Ok(x),
        Err(_) => Err(FileLoadError::InvalidJson),
    }
}

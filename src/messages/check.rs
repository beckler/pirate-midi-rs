use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CheckResponse {
    #[serde(alias = "uId")]
    pub uid: String,
    pub device_model: String,
    pub firmware_version: String,
    pub hardware_version: String,
    pub device_name: String,
    pub profile_id: String,
}

#[cfg(test)]
mod tests {
    use std::{env, path::PathBuf};

    use crate::check::CheckResponse;

    #[test]
    fn test_parsing_global_messages() {
        match env::var("CARGO_MANIFEST_DIR") {
            Ok(path) => {
                let test_file_path =
                    PathBuf::from(format!("{path}/resources/test/check-example1.json"));
                println!("manifest: {}", test_file_path.display());
                let contents =
                    std::fs::read_to_string(&test_file_path).expect("unable to read file");
                let response: Result<CheckResponse, serde_json::Error> =
                    serde_json::from_str(&contents);
                match response {
                    // if we didn't panic, we passed
                    Ok(result) => println!("{:?}", result),
                    Err(e) => panic!("parsing of response failed! - {:?}", e),
                };
            }
            Err(err) => panic!("unable to load CARGO_MANIFEST_DIR: {:?}", err),
        }
    }
}

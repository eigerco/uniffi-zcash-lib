use std::collections::HashMap;
use std::fs::read_to_string;
extern crate serde_json;

type KeyValMap = HashMap<String, String>;

pub struct TestSupport {
    map: KeyValMap,
}

impl TestSupport {
    pub fn from_csv_file() -> Self {
        let base_dir = env!("CARGO_MANIFEST_DIR");
        let csv_path = format!("{base_dir}/tests/test_data.csv");

        let map: KeyValMap = read_to_string(csv_path)
            .expect("cannot find test data")
            .split('\n')
            .map(|line| {
                if line.is_empty() {
                    ("".to_string(), "".to_string())
                } else {
                    let (k, v) = line.split_once(':').unwrap();
                    (k.to_string(), v.to_string())
                }
            })
            .collect();

        Self { map }
    }

    pub fn get_as_byte_array(&self, key: &str) -> Vec<u8> {
        let arr_str = &self.map[key];
        serde_json::from_str::<Vec<u8>>(arr_str).unwrap()
    }

    pub fn get_as_integer_array(&self, key: &str) -> Vec<u32> {
        let arr_str = &self.map[key];
        serde_json::from_str::<Vec<u32>>(arr_str).unwrap()
    }

    pub fn get_as_integer(&self, key: &str) -> u32 {
        self.map[key].parse().unwrap()
    }

    pub fn get_as_string(&self, key: &str) -> String {
        self.map[key].to_string()
    }
}

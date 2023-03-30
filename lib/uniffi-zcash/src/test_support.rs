use std::collections::HashMap;
use std::fs::read_to_string;
extern crate serde_json;

type KeyValMap = HashMap<String, String>;

pub struct TestSupport {
    map: KeyValMap
}

impl TestSupport {
    pub fn from_csv_file() -> Self {

        let base_dir = env!("CARGO_MANIFEST_DIR");
        let csv_path = format!("{base_dir}/tests/test_data.csv");

        let mut map: KeyValMap = HashMap::new();

        read_to_string(csv_path)
            .expect("cannot find test data")
            .split('\n')
            .for_each(|line| {
                let (k, v) = line.split_once(':').unwrap();
                map.insert(String::from(k), String::from(v));
            });

        Self { map }
    }

    pub fn get_as_byte_array(&self, key: &str) -> Vec<u8> {
        let arr_str = &self.map[key];
        serde_json::from_str::<Vec<u8>>(arr_str).unwrap()
    }

    pub fn get_as_string_array(&self, key: &str) -> Vec<String> {
        let arr_str = &self.map[key];
        serde_json::from_str::<Vec<String>>(arr_str).unwrap()
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
use std::collections::HashMap;
use std::fs::read_to_string;
// use crate::ZcashError;
extern crate serde_json;

type OutputsMap = HashMap<String, Vec<u8>>;
type ArgsMap = HashMap<String, Vec<String>>;

pub struct ZcashTestSupport {
    outputs: OutputsMap,
    args: ArgsMap,
}

impl ZcashTestSupport {
    pub fn from_csv_file() -> Self {
        let mut outputs: OutputsMap = HashMap::new();
        let mut args: ArgsMap = HashMap::new();

        read_to_string("tests/test_data.csv")
            .expect("cannot find test data")
            .split("\n")
            .for_each(|line| {
                if !line.is_empty() {
                    let mut iter = line.splitn(3, ';');
                    let method = iter.next().unwrap();
                    let args_str = iter
                        .next()
                        .unwrap()
                        .split('$')
                        .map(String::from)
                        .collect::<Vec<String>>();
                    let output = iter.next().unwrap();
                    outputs.insert(
                        method.to_string(),
                        serde_json::from_str::<Vec<u8>>(output).unwrap(),
                    );
                    args.insert(method.to_string(), args_str);
                }
            });

        Self { outputs, args }
    }

    pub fn get_output_as_bytes(&self, method: &str) -> Vec<u8> {
        if let Some(output) = self.outputs.get(method) {
            output.to_vec()
        } else {
            panic!("the searched method doesn't exist!")
        }
    }

    pub fn get_argument_as_string(&self, method: &str, idx: u32) -> String {
        if let Some(args) = self.args.get(method) {
            args[usize::try_from(idx).unwrap()].to_string()
        } else {
            panic!("the searched method doesn't exist!")
        }
    }

    pub fn get_argument_as_integer(&self, method: &str, idx: u32) -> u32 {
        if let Some(args) = self.args.get(method) {
            args[usize::try_from(idx).unwrap()].parse().unwrap()
        } else {
            panic!("the searched method doesn't exist!")
        }
    }

    pub fn get_argument_as_byte_array(&self, method: &str, idx: u32) -> Vec<u8> {
        if let Some(args) = self.args.get(method) {
            serde_json::from_str::<Vec<u8>>(&args[usize::try_from(idx).unwrap()]).unwrap()
        } else {
            panic!("the searched method doesn't exist!")
        }
    }
}

pub struct ZcashSpecificTestSupport {
    output: String,
    args: String,
}

impl ZcashSpecificTestSupport {
    pub fn from_method(method: &str) -> Self {
        let mut output = "";
        let mut args = "";

        let binding = read_to_string("tests/test_data.csv").expect("cannot find test data");

        binding.split("\n").for_each(|line| {
            if !line.is_empty() {
                let mut iter = line.splitn(3, ';');
                let method_int = iter.next().unwrap();

                if method == method_int {
                    args = iter.next().unwrap();
                    output = iter.next().unwrap();
                }
            }
        });

        Self {
            output: output.to_string(),
            args: args.to_string(),
        }
    }

    pub fn get_output_as_bytes(&self) -> Vec<u8> {
        if let Ok(array) = serde_json::from_str::<Vec<u8>>(&self.output) {
            array.to_vec()
        } else {
            panic!("the output doesn't exist!")
        }
    }

    pub fn get_argument_as_string(&self, idx: u32) -> String {
        let args = self
            .args
            .split('$')
            .map(String::from)
            .collect::<Vec<String>>();

        if let Some(arg) = args.get(usize::try_from(idx).unwrap()) {
            arg.to_string()
        } else {
            panic!("the argument index doesn't exist!")
        }
    }

    pub fn get_argument_as_integer(&self, idx: u32) -> u32 {
        let args = self
            .args
            .split('$')
            .map(String::from)
            .collect::<Vec<String>>();

        if let Some(arg) = args.get(usize::try_from(idx).unwrap()) {
            arg.parse().unwrap()
        } else {
            panic!("the argument index doesn't exist!")
        }
    }

    pub fn get_argument_as_byte_array(&self, idx: u32) -> Vec<u8> {
        let args = self
            .args
            .split('$')
            .map(String::from)
            .collect::<Vec<String>>();

        if let Some(arg) = args.get(usize::try_from(idx).unwrap()) {
            serde_json::from_str::<Vec<u8>>(&arg).unwrap()
        } else {
            panic!("the argument index doesn't exist!")
        }
    }
}

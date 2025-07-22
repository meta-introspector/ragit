use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct ParsedArgs {
    pub args: Vec<String>,
    pub arg_flags: HashMap<String, String>,
    pub flags: Vec<String>,
}

impl ParsedArgs {
    pub fn new() -> Self {
        ParsedArgs::default()
    }

    pub fn show_help(&self) -> bool {
        self.flags.contains(&String::from("--help"))
    }

    pub fn get_args(&self) -> Vec<String> {
        self.args.clone()
    }

    pub fn get_args_exact(&self, n: usize) -> Result<Vec<String>, String> {
        if self.args.len() == n {
            Ok(self.args.clone())
        } else {
            Err(format!("Expected {} arguments, got {}", n, self.args.len()))
        }
    }

    pub fn get_flag(&self, n: usize) -> Option<String> {
        self.flags.get(n).cloned()
    }
}

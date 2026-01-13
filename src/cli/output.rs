use serde::Serialize;

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Human,
    Json,
}

pub struct Output {
    format: OutputFormat,
    quiet: bool,
}

impl Output {
    pub fn new(json: bool, quiet: bool) -> Self {
        Self {
            format: if json {
                OutputFormat::Json
            } else {
                OutputFormat::Human
            },
            quiet,
        }
    }

    pub fn print<T: Serialize + HumanReadable>(&self, item: &T) {
        if self.quiet {
            return;
        }
        match self.format {
            OutputFormat::Human => println!("{}", item.to_human()),
            OutputFormat::Json => {
                println!("{}", serde_json::to_string_pretty(item).unwrap_or_default())
            }
        }
    }

    pub fn is_json(&self) -> bool {
        matches!(self.format, OutputFormat::Json)
    }
}

pub trait HumanReadable {
    fn to_human(&self) -> String;
}

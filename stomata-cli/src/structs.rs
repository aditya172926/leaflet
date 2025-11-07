use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "stomata")]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, default_value_t = 1000)]
    pub interval: u64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Page {
    System,
    Metrics,
}

impl Page {
    pub fn titles() -> Vec<&'static str> {
        vec!["System", "Metrics"]
    }

    pub fn get_title(&self) -> &'static str {
        match self {
            Page::System => "System",
            Page::Metrics => "Metrics",
        }
    }

    pub fn from_index(index: usize) -> Self {
        match index {
            0 => Page::System,
            1 => Page::Metrics,
            _ => Page::System,
        }
    }
}

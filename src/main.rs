use clap::{error::Result, Parser};
use serde::{Deserialize, Serialize};
use translate::translate;
use std::fs;
use std::path::PathBuf;

mod translate;

#[derive(Parser)]
#[command(
    name = "colorgen",
    version = "0.1.1",
    about = "a cli app to generate colors based on a theme file.",
)]
struct Cli {
    #[arg(short, long, help = "template dir path")]
    template: Option<PathBuf>,

    #[arg(short, long, help = "output dir path")]
    output: Option<PathBuf>,

    #[arg(short = 'T', long, help = "theme txt file path")]
    theme_txt: Option<PathBuf>,

    #[arg(short, help = "use ONLY when you want to use config system.", action = clap::ArgAction::SetTrue)]
    config: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    theme: String,
    general: General,
}

#[derive(Debug, Serialize, Deserialize)]
struct General {
    templates_path: String,
    output_path: String,
    themes_path: String,
}

impl Config {
    fn default() -> Self {
        Config {
            theme: "monochrome".to_string(),
            general: General {
                templates_path: dirs::home_dir()
                    .unwrap()
                    .join(".config/colorgen/templates")
                    .to_str()
                    .unwrap()
                    .to_string(),
                output_path: dirs::home_dir()
                    .unwrap()
                    .join(".config/colorgen/output")
                    .to_str()
                    .unwrap()
                    .to_string(),
                themes_path: dirs::home_dir()
                    .unwrap()
                    .join(".config/colorgen/themes")
                    .to_str()
                    .unwrap()
                    .to_string(),
            },
        }
    }
    fn load() -> Result<Self> {
        let config_dir = dirs::config_dir()
            .expect("cannot find config dir")
            .join("colorgen");
        fs::create_dir_all(&config_dir).expect("cannot create config dir");
        let config_file = config_dir.join("config.toml");
        if !config_file.exists() {
            let default_config = Config::default();
            let toml_str =
                toml::to_string_pretty(&default_config).expect("failed to write default config");
            let _ = fs::write(&config_file, toml_str);
        }
        let contents = fs::read_to_string(&config_file).expect("Unable to read config file");
        let toml_config: Config = toml::from_str(&contents).expect("Unable to parse config file");
        let templ_path = toml_config.general.templates_path.clone();
        let output_path = toml_config.general.output_path.clone();
        let themes_path = toml_config.general.themes_path.clone();
        if !PathBuf::from(&templ_path).exists() {
            fs::create_dir_all(&templ_path).expect("Unable to create templates dir");
        }
        if !PathBuf::from(&output_path).exists() {
            fs::create_dir_all(&output_path).expect("Unable to create output dir");
        }
        if !PathBuf::from(&themes_path).exists() {
            fs::create_dir_all(&themes_path).expect("Unable to create themes dir");
        }
        Ok(toml_config)
    }
}

fn main() {
    let cli = Cli::parse();
    let template = cli.template.unwrap_or("none".into());
    let generated = cli.output.unwrap_or("none".into());
    let theme_txt = cli.theme_txt.unwrap_or("none".into());
    let use_config = cli.config.unwrap_or(false);
    println!("template: {:?}", &template);
    println!("generated: {:?}", &generated);
    println!("theme_txt: {:?}", &theme_txt);
    println!("config: {:?}", &use_config);
    println!("----------------------");

    // let theme_file = fs::read_to_string(&theme_txt).expect("Unable to read theme_txt file");
    // println!("{}", theme_file.lines().count());
    // for line in theme_file.lines() {
    //     println!("line: {}", line);
    //     let mut word = line.split(" -> ");
    //     println!("word: {:?}", word.next().unwrap().trim_matches('$'));
    // }

    if use_config {
        let config = Config::load().expect("Unable to load config file");
        let tem_dir = fs::read_dir(&config.general.templates_path).expect("Unable to read templates dir");
        let output_dir = PathBuf::from(&config.general.output_path);
        let themes_dir = PathBuf::from(&config.general.themes_path);
        let theme_path = themes_dir.join(&config.theme);
        if !theme_path.exists() {
            panic!("Theme file does not exist");
        }
        let theme_file = theme_path.join("theme.txt");
        for entry in tem_dir {
            let entry = entry.expect("Unable to read entry");
            let file_name = entry.file_name();
            // println!("{:?}", &file_name);
            translate(&entry.path(), &file_name.to_str().unwrap(), &theme_file, &output_dir);
        }
    } else {
        let tem_dir = fs::read_dir(&template).expect("unable to read templates dir");
        for entry in tem_dir {
            let entry = entry.expect("unable to read entry");
            let file_name = entry.file_name();
            translate(&entry.path(), &file_name.to_str().unwrap(), &theme_txt, &generated);

        }
    }
    println!("Files Generated Successfully.");
}

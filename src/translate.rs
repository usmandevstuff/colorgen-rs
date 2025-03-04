use std::path::PathBuf;
use std::fs;

pub fn translate(file_path: &PathBuf, file_name: &str, theme_file: &PathBuf, generated: &PathBuf) {
    // println!("--------------------------");
    // println!("translate function called.");
    // println!("file_path: {:?}", file_path);
    // println!("file_name: {:?}", file_name);
    // println!("theme_file: {:?}", theme_file);
    // println!("generated: {:?}", generated);
    // println!("--------------------------");

    let mut templatefile = fs::read_to_string(&file_path).expect("unable to read template file in translate function");
    // let mut templatefile = fs::read_to_string("/home/usman/.config/colorgen/templates/custom.css").expect("unable to read template file in translate function");
    let themefile = fs::read_to_string(&theme_file).expect("unable to read themefile in translate function");
    // println!("{}", themefile.lines().count());
    for line in themefile.lines() {
        let mut word = line.split(" -> ");
        let var_name = word.next().unwrap().trim_matches('$');
        let new_var = format!("--{}--", var_name);
        let color_code = word.next().unwrap().trim_matches(';');
        // println!("word: {:?}", &var_name);
        // println!("color code: {:?}", &color_code);
        // println!("new_var: {:?}", &new_var);
        templatefile = templatefile.replace(&new_var, color_code);

    }
    println!("template: {}\n{}", &file_name, templatefile);
    if generated == &PathBuf::from("none") {
        println!("generated path is none");
        return;
    }
    fs::write(generated.join(file_name), templatefile).expect("unable to write to generated file");
}

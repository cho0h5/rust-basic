use std::fs;
use glob::glob;

fn main() -> std::io::Result<()> {
    //let PATH = "../*/*/.git";
    let PATH = "./*/*/.git";

    for e in glob(PATH).expect("Failed to read glob pattern") {
        let path = e.unwrap().display().to_string();
        println!("{}", path);
        fs::remove_dir_all(path)?;
    }
    Ok(())
}

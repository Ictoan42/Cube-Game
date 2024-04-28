use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=src/res/font/");

    let font = std::fs::read_dir("./src/res/font/").unwrap().next().unwrap().unwrap();
    let font_path = font.path().to_str().unwrap().to_string();

    for i in 0..=9 {
        let c = format!("
            magick \
            -background transparent \
            -fill black \
            -font {font_path} \
            -size 128x256 \
            -gravity center \
            label:{i} \
            -blur 0x1 \
            ./src/res/gen/{i}.png
        ");

        let _ = Command::new("sh")
        .arg("-c")
        .arg(c.clone())
        .output()
        .expect("Failed to execute");
    }
}

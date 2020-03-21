use std::fs;

pub fn display_menu() {
    let file_path = "server/assets/menu_text.txt";
    let menu_contents: String = fs::read_to_string(file_path)
        .expect(format!("Unable to open file {}", file_path).as_ref())
        .parse()
        .expect(format!("Error parsing file contents for {}.", file_path).as_ref());


    println!("{}", menu_contents);
}

const META_HEADER: &str = r#"---@meta

---@class laravox
laravox = {}

"#;
const META_FILE: &str = "meta.lua";
const META_PATH: &str = "../engine_macro/out";
const MAIN_PATH: &str = "../main/";

fn main() {
    let mut buffer = self::META_HEADER.to_string();

    for path in std::fs::read_dir(self::META_PATH).unwrap() {
        let path = path.unwrap().path();
        let file = std::fs::read_to_string(&path).unwrap();
        std::fs::remove_file(&path).unwrap();

        buffer.push_str(&format!("{}\n\n", file.trim()));
    }

    let path = std::path::Path::new(self::MAIN_PATH);

    if path.is_dir() {
        std::fs::write(format!("{}{}", self::MAIN_PATH, self::META_FILE), buffer).unwrap();
    }
}

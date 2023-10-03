use std::fs::read_dir;
use std::io::Write;

use goldenfile::mint::Mint;

const SINGLE_BIN_INPUT_DIR: &str = "tests/examples/single-binaries";
const OUTPUT_DIR: &str = "tests/goldenfiles";

#[test]
fn single_binaries() {
    let mut mint = Mint::new(OUTPUT_DIR);

    for input_dir_res in read_dir(SINGLE_BIN_INPUT_DIR).unwrap() {
        let input_dir = input_dir_res.unwrap();
        let name = input_dir
            .path()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let bundled = cargo_bundler::bundle_project(&input_dir.path(), None)
            .expect("Could not bundle project");

        let mut goldenfile = mint
            .new_goldenfile(&format!("{name}.rs"))
            .expect("Could not find goldenfile");
        write!(goldenfile, "{}", &bundled).unwrap();
    }
}

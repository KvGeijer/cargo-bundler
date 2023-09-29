use std::{fs, path::Path};

mod cargo_parser;
mod flattener;

pub type Result<T> = std::result::Result<T, String>;

pub fn bundle_project(package_path: &Path, bin_target: Option<String>) -> Result<String> {
    let (bin_path, opt_lib_path) =
        cargo_parser::parse_access_paths(package_path, bin_target.as_deref())?;
    // Ok(format!())
    println!("Found cargo {:?} {:?}", bin_path, opt_lib_path);

    let opt_flattened_lib = opt_flatten_library(opt_lib_path.as_deref())?;

    let flattened_bin = flatten_binary(&bin_path)?;
    Ok(format!(
        "Library:\n{}\n\n\nBinary:\n{}",
        opt_flattened_lib
            .map(|ast| prettyplease::unparse(&ast))
            .unwrap_or("Not found".to_string()),
        prettyplease::unparse(&flattened_bin)
    ))
}

fn flatten_binary(bin_path: &Path) -> Result<syn::File> {
    let bin_file = fs::read_to_string(bin_path)
        .map_err(|err| format!("Could not read binary file: {}", err.to_string()))?;
    let tree = syn::parse_str(&bin_file).map_err(|err| {
        format!(
            "Could not parse binary into syntax tree: {}",
            err.to_string()
        )
    })?;

    flattener::flatten_ast(
        tree,
        bin_path
            .parent()
            .expect("Bin file should be in a directory"),
    )
}

fn opt_flatten_library(opt_lib_path: Option<&Path>) -> Result<Option<syn::File>> {
    if let Some(lib_path) = opt_lib_path {
        let lib_file = fs::read_to_string(lib_path)
            .map_err(|err| format!("Could not read library file: {}", err.to_string()))?;
        let tree = syn::parse_str(&lib_file).map_err(|err| {
            format!(
                "Could not parse library into syntax tree: {}",
                err.to_string()
            )
        })?;

        Ok(Some(flattener::flatten_ast(
            tree,
            lib_path
                .parent()
                .expect("Bin file should be in a directory"),
        )?))
    } else {
        Ok(None)
    }
}

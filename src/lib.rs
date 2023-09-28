use std::path::Path;

mod cargo_parser;

pub fn bundle_project(package_path: &Path, bin_target: Option<String>) -> Result<String, String> {
    let (bin_path, opt_lib_path) =
        cargo_parser::parse_access_paths(package_path, bin_target.as_deref())?;
    Ok(format!("Found cargo {:?} {:?}", bin_path, opt_lib_path))
}

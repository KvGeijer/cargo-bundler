use std::path::{Path, PathBuf};

pub fn parse_access_paths(
    package_path: &Path,
    bin_target: Option<&str>,
) -> Result<(PathBuf, Option<(PathBuf, String)>), String> {
    let metadata = get_metadata(package_path)?;
    let targets = &metadata.root_package().unwrap().targets;
    let bin = select_binary(targets, bin_target)?;
    let lib = get_lib(targets);

    Ok((bin, lib))
}

fn get_metadata(package_path: &Path) -> Result<cargo_metadata::Metadata, String> {
    let manifest_path = package_path.join("Cargo.toml");
    cargo_metadata::MetadataCommand::new()
        .manifest_path(&manifest_path)
        .exec()
        .map_err(|err| err.to_string())
}

fn select_binary(
    targets: &[cargo_metadata::Target],
    binary: Option<&str>,
) -> Result<PathBuf, String> {
    let binary_targets: Vec<_> = targets.iter().filter(|t| target_is(t, "bin")).collect();
    if binary_targets.is_empty() {
        return Err(
            "No binary target found. Don't support library only applications (yet)".to_string(),
        );
    }

    if let Some(chosen_binary) = binary {
        binary_targets
            .into_iter()
            .find(|t| t.name.eq(&chosen_binary))
            .ok_or_else(|| format!("Cannot find the chosen binary target \"{chosen_binary}\""))
            .map(|target| target.src_path.clone().into())
    } else if binary_targets.len() > 1 {
        Err("Multiple binary targets detected. Please specify which one to use".to_string())
    } else {
        Ok(binary_targets[0].src_path.clone().into())
    }
}

fn target_is(target: &cargo_metadata::Target, target_kind: &str) -> bool {
    target.kind.iter().any(|kind| kind == target_kind)
}

fn get_lib(targets: &[cargo_metadata::Target]) -> Option<(PathBuf, String)> {
    let libs: Vec<_> = targets.iter().filter(|t| target_is(t, "lib")).collect();
    assert!(
        libs.len() <= 1,
        "Multiple library targets found. Not supported." // This should not be possible
    );

    libs.into_iter().next().map(|target| {
        (
            target.src_path.clone().into(),
            target.name.replace("-", "_").clone(),
        )
    })
}

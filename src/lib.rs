use std::{fs, path::Path};

mod cargo_parser;
mod crate_path_injecter;
mod flattener;
mod linker;

pub type Result<T> = std::result::Result<T, String>;

pub fn bundle_project(package_path: &Path, bin_target: Option<String>) -> Result<String> {
    let (bin_path, opt_lib_info) =
        cargo_parser::parse_access_paths(package_path, bin_target.as_deref())?;

    let flattened_bin = flatten_source(&bin_path)?;

    // Flatten an link the library
    if let Some((lib_path, lib_name)) = opt_lib_info {
        let flattened_lib = flatten_source(&lib_path)?;

        let merged_ast = merge_into(flattened_bin, flattened_lib, &lib_name)?;

        Ok(prettyplease::unparse(&merged_ast))
    } else {
        Ok(prettyplease::unparse(&flattened_bin))
    }
}

fn flatten_source(source_path: &Path) -> Result<syn::File> {
    let source_file = fs::read_to_string(source_path)
        .map_err(|err| format!("Could not read source file: {}", err.to_string()))?;
    let tree = syn::parse_str(&source_file).map_err(|err| {
        format!(
            "Could not parse source code into syntax tree: {}",
            err.to_string()
        )
    })?;

    flattener::flatten_ast(
        tree,
        source_path
            .parent()
            .expect("Source file should be in a directory"),
    )
}

fn merge_into(
    target_ast: syn::File,
    merging_ast: syn::File,
    merging_name: &str,
) -> Result<syn::File> {
    // Update all paths in merging ast which use crate, as they will be invalidated during merge
    let inj_merging_ast = crate_path_injecter::inject_crate_paths(merging_ast, merging_name)?;

    // Create a new mod item, to contain the flattened library
    let name_ident: syn::Ident = syn::parse_str(merging_name).expect("Failed to parse identifier");
    let mut merge_mod: syn::ItemMod = syn::parse_quote! {
        mod temp {}
    };
    merge_mod.ident = name_ident;

    // TODO: Inherit attributes from library into main
    // Move all everything from the merging ast into a new module, with the merging_name
    if let Some((_, ref mut items)) = &mut merge_mod.content {
        *items = inj_merging_ast.items;
    }

    // Link in the library module to all modules in the binary tree
    let mut linked_target_ast = linker::link_ast(target_ast, merging_name)?;

    // Merge the new module into the target ast
    linked_target_ast.items.insert(0, merge_mod.into());

    Ok(linked_target_ast)
}

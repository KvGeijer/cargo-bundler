use syn::visit_mut::VisitMut;

use super::Result;
use std::{
    fs,
    path::{Path, PathBuf},
    process::exit,
};

/// Inlines all mod declarations in the ast. Needs the dir of the source file in the ast, ast to be able to find module files
pub fn flatten_ast(mut ast: syn::File, ast_dir_path: &Path) -> Result<syn::File> {
    let mut flattener = Flattener {
        mod_folder_path: ast_dir_path,
    };
    flattener.visit_file_mut(&mut ast);
    Ok(ast)
}

struct Flattener<'a> {
    mod_folder_path: &'a Path,
}

// TODO: Can probably be optimized by eg. not visiting expressions.
impl<'a> VisitMut for Flattener<'a> {
    fn visit_item_mod_mut(&mut self, item: &mut syn::ItemMod) {
        syn::visit_mut::visit_item_mod_mut(self, item);

        // Inline the module into the syntax tree
        if let Err(reason) = self.inline_mod(item) {
            // TODO: Not very nice to error out here when we propagate results in all other parts of the code...
            eprintln!("FLATTENING ERROR: {reason}");
            exit(1)
        };
    }
}

impl<'a> Flattener<'a> {
    fn inline_mod(&self, item: &mut syn::ItemMod) -> Result<()> {
        if item.content.is_some() {
            // Already inlined
            return Ok(());
        }

        let mod_name = item.ident.to_string();
        let mod_path = find_mod_file(&mod_name, self.mod_folder_path)?;
        let mod_folder = self.mod_folder_path.join(mod_name);

        let mod_file_str = fs::read_to_string(&mod_path)
            .map_err(|err| format!("Could not read mod file: {}", err.to_string()))?;

        let mod_ast = syn::parse_str(&mod_file_str).map_err(|err| {
            format!(
                "Could not parse module {} into syntax tree: {}",
                mod_path.as_path().to_str().unwrap(),
                err.to_string()
            )
        })?;

        let flattened_ast = flatten_ast(mod_ast, &mod_folder)?;

        // TODO: Here we just include the items. Do we get rid of any important metadata such as attributes?
        item.content = Some((Default::default(), flattened_ast.items));
        Ok(())
    }
}

/// Finds the path to a module file. Looks for both the `mod.rs` version, and a file with the same name in the same folder
fn find_mod_file(mod_name: &str, dir: &Path) -> Result<PathBuf> {
    let is_mod = mod_name.eq("mod");
    let mod_code_file_name = format!("{mod_name}.rs");
    for entry in dir
        .read_dir()
        .expect("Could not read parent dir of file when flattening")
        .into_iter()
    {
        if let Ok(direntry) = entry {
            let code_name = direntry
                .file_name()
                .as_os_str()
                .to_str()
                .unwrap() // Lot's of these unsafe conversions from OsString...
                .replace("-", "_");
            if let Ok(dirtype) = direntry.file_type() {
                // Is there a mod.rs file within the directory with correct mod name?
                if dirtype.is_dir() && code_name.eq(mod_name) && !is_mod {
                    if let Ok(path) = find_mod_file("mod", &direntry.path()) {
                        return Ok(path);
                    }
                } else if dirtype.is_file() && code_name.eq(&mod_code_file_name) {
                    // Found a file with corresponding name
                    return Ok(direntry.path());
                }
            }
        }
    }
    Err(format!(
        "FLATTENING ERROR: No file found for module {}, which is in turn within folder {}",
        mod_name,
        dir.to_str().unwrap()
    ))
}

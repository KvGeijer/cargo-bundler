use syn::visit_mut::VisitMut;

use crate::Result;

pub fn link_ast(mut ast: syn::File, module: &str) -> Result<syn::File> {
    // Create a new "use crate::${module};" syn item to insert into every module
    let mut fake_use_lib: syn::ItemUse = syn::parse_quote! {
        use crate::temp;
    };

    if let syn::UseTree::Path(ref mut path) = &mut fake_use_lib.tree {
        let new_tree = syn::UseTree::Name(syn::UseName {
            ident: syn::Ident::new(module, path.ident.span().clone()),
        });
        path.tree = Box::new(new_tree);
    } else {
        panic!("could not parse use item")
    }

    let mut linker = Linker {
        use_item: fake_use_lib,
    };

    linker.visit_file_mut(&mut ast);

    Ok(ast)
}

/// Visits all inline modules and adds a the use_item to each of those modules, creating a link as if to an external crate or library
struct Linker {
    use_item: syn::ItemUse,
}

// TODO: Can probably be optimized by eg. not visiting expressions.
impl VisitMut for Linker {
    fn visit_item_mod_mut(&mut self, item: &mut syn::ItemMod) {
        syn::visit_mut::visit_item_mod_mut(self, item);

        // Add the use item
        match &mut item.content {
            Some((_, ref mut items)) => items.insert(0, self.use_item.clone().into()),
            None => panic!("Must have flattened the ast before linking!"),
        }
    }
}

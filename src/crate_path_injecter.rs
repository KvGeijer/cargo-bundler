use syn::visit_mut::VisitMut;

use crate::Result;

/// Injects the injection string after each "crate" in all paths/use paths in the ast
pub fn inject_crate_paths(mut ast: syn::File, injection: &str) -> Result<syn::File> {
    let mut injector = Injector {
        injection: injection.to_string(),
    };

    injector.visit_file_mut(&mut ast);

    Ok(ast)
}

struct Injector {
    injection: String,
}

impl VisitMut for Injector {
    // Injects a path segment after all normal (linear) paths after potential "crate::"
    fn visit_path_mut(&mut self, path: &mut syn::Path) {
        if let Some(segment) = path.segments.first() {
            if segment.ident.eq("crate") {
                let mut injection: syn::PathSegment = segment.clone();
                injection.ident = syn::Ident::new(&self.injection, injection.ident.span().clone());

                path.segments.insert(1, injection);
            }
        }
        // TODO: Is this needed?
        syn::visit_mut::visit_path_mut(self, path);
    }

    // Injects a path segment after paths starting with crate in use paths
    fn visit_use_path_mut(&mut self, path: &mut syn::UsePath) {
        if path.ident.eq("crate") {
            // Create a new use path to inject
            let mut injection = path.clone();
            injection.ident = syn::Ident::new(&self.injection, path.ident.span());

            path.tree = Box::new(syn::UseTree::Path(injection));
        }

        // Needed in case of groupings or something
        syn::visit_mut::visit_use_path_mut(self, path);
    }
}

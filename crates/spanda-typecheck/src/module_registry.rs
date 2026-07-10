//! Multi-file module export registry for type checking linked projects.
//!
use spanda_ast::foundations::TraitDecl;
use spanda_ast::foundations::{ModuleFnDecl, Visibility};
use spanda_ast::nodes::Program;
use std::collections::HashMap;

/// Exported symbols from a single module.
#[derive(Debug, Clone, Default)]
pub struct ModuleExports {
    pub functions: HashMap<String, ModuleFnDecl>,
    pub traits: HashMap<String, TraitDecl>,
}

/// Registry of parsed modules keyed by fully-qualified module name.
#[derive(Debug, Clone, Default)]
pub struct ModuleRegistry {
    modules: HashMap<String, ModuleExports>,
}

impl ModuleRegistry {
    pub fn new() -> Self {
        // Description:
        //     Construct a new instance.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     result: Self
        //         Return value from `new`.
        //
        // Example:

        //     let value = spanda_typecheck::module_registry::new();

        Self::default()
    }

    pub fn register(&mut self, module_name: &str, program: &Program) {
        // Description:
        //     Register.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     odule_name: &str
        //         Caller-supplied odule name.
        //     progra: &Program
        //         Caller-supplied progra.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_typecheck::module_registry::register(&mut self, odule_name, progra);

        let Program::Program {
            functions, traits, ..
        } = program;
        let mut exports = ModuleExports::default();
        for func in functions {
            let ModuleFnDecl {
                name, visibility, ..
            } = func;
            if matches!(visibility, Visibility::Public | Visibility::Export) {
                exports.functions.insert(name.clone(), func.clone());
            }
        }

        // Export public/export traits for cross-module `impl` resolution.
        for trait_decl in traits {
            let TraitDecl::TraitDecl {
                name, visibility, ..
            } = trait_decl;
            if matches!(visibility, Visibility::Public | Visibility::Export) {
                exports.traits.insert(name.clone(), trait_decl.clone());
            }
        }
        self.modules.insert(module_name.to_string(), exports);
    }

    pub fn from_programs(entries: &[(String, Program)]) -> Self {
        // Description:
        //     From programs.
        //
        // Inputs:
        //     entries: &[(String, Program)]
        //         Caller-supplied entries.
        //
        // Outputs:
        //     result: Self
        //         Return value from `from_programs`.
        //
        // Example:

        //     let result = spanda_typecheck::module_registry::from_programs(entries);

        let mut registry = Self::new();
        for (name, program) in entries {
            registry.register(name, program);
        }
        registry
    }

    pub fn exports_for(&self, import_path: &str) -> Option<&ModuleExports> {
        // Description:
        //     Exports for.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     import_path: &str
        //         Caller-supplied import path.
        //
        // Outputs:
        //     result: Option<&ModuleExports>
        //         Return value from `exports_for`.
        //
        // Example:

        //     let result = spanda_typecheck::module_registry::exports_for(&self, import_path);

        self.modules.get(import_path)
    }

    pub fn function(&self, import_path: &str, name: &str) -> Option<&ModuleFnDecl> {
        // Description:
        //     Function.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     import_path: &str
        //         Caller-supplied import path.
        //     name: &str
        //         Caller-supplied name.
        //
        // Outputs:
        //     result: Option<&ModuleFnDecl>
        //         Return value from `function`.
        //
        // Example:

        //     let result = spanda_typecheck::module_registry::function(&self, import_path, name);

        self.exports_for(import_path)
            .and_then(|e| e.functions.get(name))
    }

    pub fn module_count(&self) -> usize {
        // Description:
        //     Module count.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: usize
        //         Return value from `module_count`.
        //
        // Example:

        //     let result = spanda_typecheck::module_registry::module_count(&self);

        self.modules.len()
    }
}

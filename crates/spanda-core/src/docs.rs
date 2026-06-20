//! Markdown documentation generator for Spanda programs.
//!
//! Emits module-level API reference from the AST: imports, functions, structs,
//! enums, traits, robots, and test blocks.

use crate::ast::*;
use crate::error::SpandaError;
use crate::foundations::Visibility;

pub fn generate_markdown(source: &str) -> Result<String, SpandaError> {
    // Generate Markdown API documentation from Spanda source.
    //
    // Parameters:
    //
    // - `source` — Full program source text.
    //
    // Returns:
    //
    // Markdown document string, or [`SpandaError`] if lexing/parsing fails.
    //
    // Example:
    //
    // use spanda_core::docs::generate_markdown;
    // let source = r#"
    // module nav;
    // export fn plan() -> Path { return trajectory(from: pose(x: 0.0 m, y: 0.0 m), to: pose(x: 1.0 m, y: 0.0 m), steps: 3); }
    // robot R { actuator wheels: DifferentialDrive; behavior run() { wheels.stop(); } }
    // "#;
    // let md = generate_markdown(source).unwrap();
    // assert!(md.contains("# Module `nav`"));
    // assert!(md.contains("### `R`"));
    let tokens = crate::lexer::tokenize(source)?;
    let program = crate::parser::parse(tokens)?;
    Ok(render_program_docs(&program))
}

fn render_program_docs(program: &Program) -> String {
    // Render program docs.
    //
    // Parameters:
    // - `program` — input value
    //
    // Returns:
    // Text result.
    //
    // Options:
    // None.
    //
    // Example:
    // let result = spanda_core::docs::render_program_docs(program);

    let Program::Program {
        module_name,
        imports,
        functions,
        tests,
        structs,
        enums,
        traits,
        robots,
        ..
    } = program;

    let mut out = String::new();
    let title = module_name
        .as_deref()
        .unwrap_or("anonymous")
        .replace('.', "/");
    out.push_str(&format!("# Module `{title}`\n\n"));

    if !imports.is_empty() {
        out.push_str("## Imports\n\n");
        for import in imports {
            let ImportDecl::ImportDecl { path, .. } = import;
            out.push_str(&format!("- `{path}`\n"));
        }
        out.push('\n');
    }

    if !functions.is_empty() {
        out.push_str("## Functions\n\n");
        for func in functions {
            out.push_str(&render_module_fn(func));
            out.push('\n');
        }
    }

    if !structs.is_empty() {
        out.push_str("## Structs\n\n");
        for s in structs {
            out.push_str(&render_struct(s));
            out.push('\n');
        }
    }

    if !enums.is_empty() {
        out.push_str("## Enums\n\n");
        for e in enums {
            out.push_str(&render_enum(e));
            out.push('\n');
        }
    }

    if !traits.is_empty() {
        out.push_str("## Traits\n\n");
        for t in traits {
            out.push_str(&render_trait(t));
            out.push('\n');
        }
    }

    if !robots.is_empty() {
        out.push_str("## Robots\n\n");
        for robot in robots {
            out.push_str(&render_robot(robot));
            out.push('\n');
        }
    }

    if !tests.is_empty() {
        out.push_str("## Tests\n\n");
        for test in tests {
            out.push_str(&format!(
                "- `\"{}\"` ({} statements)\n",
                test.name,
                test.body.len()
            ));
        }
        out.push('\n');
    }

    out
}

fn render_module_fn(func: &crate::foundations::ModuleFnDecl) -> String {
    // Render module fn.
    //
    // Parameters:
    // - `func` — input value
    //
    // Returns:
    // Text result.
    //
    // Options:
    // None.
    //
    // Example:
    // let result = spanda_core::docs::render_module_fn(func);

    let visibility = match func.visibility {
        Visibility::Export => "export ",
        Visibility::Public => "public ",
        Visibility::Private => "private ",
    };
    let async_kw = if func.is_async { "async " } else { "" };
    let type_params = if func.type_params.is_empty() {
        String::new()
    } else {
        format!("<{}>", func.type_params.join(", "))
    };
    let params = func
        .params
        .iter()
        .map(|p| format!("{}: {}", p.name, type_name(&p.type_ann)))
        .collect::<Vec<_>>()
        .join(", ");
    format!(
        "### {visibility}{async_kw}fn `{name}{type_params}({params}) -> {ret}`\n",
        name = func.name,
        ret = type_name(&func.return_type),
    )
}

fn render_struct(decl: &crate::foundations::StructDecl) -> String {
    // Render struct.
    //
    // Parameters:
    // - `decl` — input value
    //
    // Returns:
    // Text result.
    //
    // Options:
    // None.
    //
    // Example:
    // let result = spanda_core::docs::render_struct(decl);

    let crate::foundations::StructDecl::StructDecl { name, fields, .. } = decl;
    let mut out = format!("### `{name}`\n\n");
    for field in fields {
        out.push_str(&format!("- `{}`: `{}`\n", field.name, field.type_name));
    }
    out
}

fn render_enum(decl: &crate::foundations::EnumDecl) -> String {
    // Render enum.
    //
    // Parameters:
    // - `decl` — input value
    //
    // Returns:
    // Text result.
    //
    // Options:
    // None.
    //
    // Example:
    // let result = spanda_core::docs::render_enum(decl);

    let crate::foundations::EnumDecl::EnumDecl { name, variants, .. } = decl;
    let mut out = format!("### `{name}`\n\n");
    for variant in variants {
        if variant.field_types.is_empty() {
            out.push_str(&format!("- `{}`\n", variant.name));
        } else {
            out.push_str(&format!(
                "- `{}({})`\n",
                variant.name,
                variant.field_types.join(", ")
            ));
        }
    }
    out
}

fn render_trait(decl: &crate::foundations::TraitDecl) -> String {
    // Render trait.
    //
    // Parameters:
    // - `decl` — input value
    //
    // Returns:
    // Text result.
    //
    // Options:
    // None.
    //
    // Example:
    // let result = spanda_core::docs::render_trait(decl);

    let crate::foundations::TraitDecl::TraitDecl { name, methods, .. } = decl;
    let mut out = format!("### `{name}`\n\n");
    for method in methods {
        let params = method
            .params
            .iter()
            .map(|p| format!("{}: {}", p.name, p.type_name))
            .collect::<Vec<_>>()
            .join(", ");
        out.push_str(&format!(
            "- `fn {}({}) -> {}`\n",
            method.name, params, method.return_type
        ));
    }
    out
}

fn render_robot(robot: &RobotDecl) -> String {
    // Render robot.
    //
    // Parameters:
    // - `robot` — input value
    //
    // Returns:
    // Text result.
    //
    // Options:
    // None.
    //
    // Example:
    // let result = spanda_core::docs::render_robot(robot);

    let RobotDecl::RobotDecl {
        name,
        sensors,
        actuators,
        agents,
        behaviors,
        tasks,
        ..
    } = robot;
    let mut out = format!("### `{name}`\n\n");
    if !sensors.is_empty() {
        out.push_str("**Sensors**\n\n");
        for sensor in sensors {
            let SensorDecl::SensorDecl {
                name, sensor_type, ..
            } = sensor;
            out.push_str(&format!("- `{name}`: `{sensor_type}`\n"));
        }
        out.push('\n');
    }
    if !actuators.is_empty() {
        out.push_str("**Actuators**\n\n");
        for actuator in actuators {
            let ActuatorDecl::ActuatorDecl {
                name,
                actuator_type,
                ..
            } = actuator;
            out.push_str(&format!("- `{name}`: `{actuator_type}`\n"));
        }
        out.push('\n');
    }
    if !agents.is_empty() {
        out.push_str("**Agents**\n\n");
        for agent in agents {
            let AgentDecl::AgentDecl { name, goal, .. } = agent;
            out.push_str(&format!("- `{name}` — goal: \"{goal}\"\n"));
        }
        out.push('\n');
    }
    if !behaviors.is_empty() {
        out.push_str("**Behaviors**\n\n");
        for behavior in behaviors {
            let BehaviorDecl::BehaviorDecl { name, .. } = behavior;
            out.push_str(&format!("- `{name}()`\n"));
        }
        out.push('\n');
    }
    if !tasks.is_empty() {
        out.push_str("**Tasks**\n\n");
        for task in tasks {
            let crate::foundations::TaskDecl::TaskDecl {
                name, interval_ms, ..
            } = task;
            out.push_str(&format!("- `{name}` every {interval_ms}ms\n"));
        }
    }
    out
}

fn type_name(ty: &SpandaType) -> String {
    // Type name.
    //
    // Parameters:
    // - `ty` — input value
    //
    // Returns:
    // Text result.
    //
    // Options:
    // None.
    //
    // Example:
    // let result = spanda_core::docs::type_name(ty);

    match ty {
        SpandaType::Void => "Void".into(),
        SpandaType::Int => "Int".into(),
        SpandaType::Float => "Float".into(),
        SpandaType::Bool => "Bool".into(),
        SpandaType::String => "String".into(),
        SpandaType::Char => "Char".into(),
        SpandaType::Bytes => "Bytes".into(),
        SpandaType::Null => "Null".into(),
        SpandaType::Number { unit } => {
            if *unit == UnitKind::None {
                "Number".into()
            } else {
                format!("Number({})", unit.as_str())
            }
        }
        SpandaType::Named { name } => name.clone(),
        SpandaType::Generic { name, type_args } => {
            let args = type_args
                .iter()
                .map(type_name)
                .collect::<Vec<_>>()
                .join(", ");
            format!("{name}<{args}>")
        }
        SpandaType::TypeParam { name } => name.clone(),
        SpandaType::Scan => "Scan".into(),
        SpandaType::Pose => "Pose".into(),
        SpandaType::Velocity => "Velocity".into(),
        SpandaType::Trajectory => "Trajectory".into(),
        SpandaType::Transform => "Transform".into(),
        SpandaType::EnumVariant { enum_name, variant } => format!("{enum_name}.{variant}"),
        SpandaType::TraitObject { trait_name } => format!("dyn {trait_name}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_module_docs() {
        // Generates module docs.
        //
        // Parameters:
        // None.
        //
        // Returns:
        // Nothing.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = spanda_core::docs::generates_module_docs();

        let source = r#"
module navigation;

export fn plan() -> Path {
  return trajectory(from: pose(x: 0.0 m, y: 0.0 m), to: pose(x: 1.0 m, y: 0.0 m), steps: 3);
}

robot R {
  actuator wheels: DifferentialDrive;
  behavior run() { wheels.stop(); }
}
"#;
        let md = generate_markdown(source).expect("docs");
        assert!(md.contains("# Module `navigation`"));
        assert!(md.contains("export fn `plan("));
        assert!(md.contains("### `R`"));
    }
}

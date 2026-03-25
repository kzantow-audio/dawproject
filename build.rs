use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;

use xsd_parser::config::{GeneratorFlags, InterpreterFlags, OptimizerFlags, Schema};
use xsd_parser::{generate, Config};

fn main() {
    let gen_dir = Path::new("src/generated");

    if std::env::var("DOCS_RS").is_err() {
        Command::new("git")
            .args(["submodule", "update", "--init", "--recursive"])
            .output()
            .expect("Sync submodules failed!");

        // MetaData.xsd
        let fixed_metadata_xsd: String = fs::read_to_string("dawproject/MetaData.xsd").unwrap();

        // Project.xsd — fix contentType → contentTypes for XSD parsing
        let project_xsd: String = fs::read_to_string("dawproject/Project.xsd").unwrap();
        let fixed_project_xsd = project_xsd.replace(
            "<xs:attribute name=\"contentType\">",
            "<xs:attribute name=\"contentTypes\">",
        );

        // Fix naming collisions: simpleType names that clash with complexType names
        let fixed_project_xsd = fixed_project_xsd
            .replace("\"eqBandType\"", "\"eqBandKind\"")
            .replace("\"sendType\"", "\"sendKind\"");

        fs::write("assets/FixedMetaData.xsd", &fixed_metadata_xsd).unwrap();
        fs::write("assets/FixedProject.xsd", &fixed_project_xsd).unwrap();
    }

    // Generate metadata types
    generate_from_xsd(
        "assets/FixedMetaData.xsd",
        &gen_dir.join("metadata_generated.rs"),
    );

    // Generate project types
    generate_from_xsd(
        "assets/FixedProject.xsd",
        &gen_dir.join("project_generated.rs"),
    );

    println!("cargo:rerun-if-changed=assets/FixedMetaData.xsd");
    println!("cargo:rerun-if-changed=assets/FixedProject.xsd");
    println!("cargo:rerun-if-changed=dawproject/MetaData.xsd");
    println!("cargo:rerun-if-changed=dawproject/Project.xsd");
}

fn generate_from_xsd(xsd_path: &str, output_path: &Path) {
    let config = Config::default()
        .with_schema(Schema::File(xsd_path.into()))
        .with_interpreter_flags(InterpreterFlags::all())
        .with_optimizer_flags(OptimizerFlags::all())
        .with_generator_flags(GeneratorFlags::all())
        .with_serde_quick_xml();

    let code = generate(config).unwrap_or_else(|e| {
        panic!("Failed to generate code from {xsd_path}: {e}");
    });
    let code = code.to_string();

    // Post-process: add Clone and PartialEq derives to all generated types
    // The token stream produces spaced-out formatting
    let code = code
        .replace(
            "# [derive (Debug , Deserialize , Serialize)]",
            "#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]",
        )
        .replace(
            "# [derive (Debug , Default , Deserialize , Serialize)]",
            "#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]",
        );

    // Post-process: add skip_serializing_if to Option and Vec fields
    // so that None/empty values are omitted in serialized XML
    let code = add_skip_serializing_if(&code);

    // Only write if content changed to avoid triggering unnecessary rebuilds
    if output_path.exists() {
        if let Ok(existing) = fs::read_to_string(output_path) {
            if existing == code {
                return;
            }
        }
    }

    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    let mut file = fs::File::create(output_path).unwrap();
    file.write_all(code.as_bytes()).unwrap();
}

/// Add `skip_serializing_if` to Option and Vec fields marked with `#[serde(default)]`.
/// This ensures None values and empty Vecs are omitted from serialized XML,
/// preventing round-trip issues (e.g., empty string "" failing to parse as i32).
fn add_skip_serializing_if(code: &str) -> String {
    let target = "# [serde (default , ";
    let mut result = String::with_capacity(code.len() + 10000);
    let mut remaining = code;

    while let Some(pos) = remaining.find(target) {
        result.push_str(&remaining[..pos]);
        let after_target = &remaining[pos + target.len()..];

        // Determine field type by looking ahead to the next serde/derive/struct boundary
        let boundary = after_target
            .find("# [")
            .or_else(|| after_target.find("pub struct "))
            .or_else(|| after_target.find("pub enum "))
            .or_else(|| after_target.find("pub type "))
            .unwrap_or(after_target.len());
        let field_slice = &after_target[..boundary];

        if field_slice.contains(":: core :: option :: Option") {
            result.push_str(
                "# [serde (default , skip_serializing_if = \"Option::is_none\" , ",
            );
        } else if field_slice.contains(":: std :: vec :: Vec") {
            result.push_str(
                "# [serde (default , skip_serializing_if = \"Vec::is_empty\" , ",
            );
        } else {
            result.push_str(target);
        }

        remaining = after_target;
    }
    result.push_str(remaining);
    result
}

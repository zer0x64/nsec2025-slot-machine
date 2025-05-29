use std::io::Result;
fn main() -> Result<()> {
    build_schemas()?;
    tauri_build::build();

    Ok(())
}

fn build_schemas() -> Result<()> {
    println!("cargo:rerun-if-changed=schemas/models.proto");

    let mut config = prost_build::Config::new();
    config.type_attribute(".", "#[derive(serde::Serialize)]");
    config.type_attribute(".", "#[serde(rename_all = \"camelCase\")]");
    config.compile_protos(&["schemas/models.proto"], &["schemas/"])?;
    Ok(())
}

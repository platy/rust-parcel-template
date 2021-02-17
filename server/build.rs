use std::{
    env,
    error::Error,
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
    str::FromStr,
};

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=build.rs");

    let (web_sources, web_artifacts) = build_web()?;

    for path in web_sources {
        println!("cargo:rerun-if-changed={:?}", path);
    }

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let mut generated_web_file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(Path::new(&out_dir).join("web.rs"))?;
    let mut artifact_map_insertions = String::new();
    for artifact in web_artifacts {
        let artifact = artifact.to_str().unwrap();
        let identifier = artifact.replace('/', "_").replace('.', "_").to_uppercase();
        writeln!(
            generated_web_file,
            r#"pub const {}: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../web/{}"));"#,
            identifier, artifact,
        )?;
        artifact_map_insertions.push_str(&format!(
            "        \"{}\" => Some({}),\n",
            artifact.strip_prefix("dist").unwrap(), identifier,
        ));
    }
    writeln!(generated_web_file,)?;
    writeln!(
        generated_web_file,
        "pub fn artifact(path: &str) -> Option<&'static[u8]> {{
    match path {{
{}
        _ => None,
    }}
}}
",
        artifact_map_insertions,
    )?;

    Ok(())
}

fn build_web() -> Result<(Vec<PathBuf>, Vec<PathBuf>), Box<dyn Error>> {
    let install_output = Command::new("npm")
        .arg("install")
        .current_dir("../web")
        .output()?;
    println!("{}", String::from_utf8(install_output.stdout)?);
    println!("{}", String::from_utf8(install_output.stderr)?);
    assert!(install_output.status.success());
    let build_output = Command::new("npm")
        .args(&["run", "build", "--", "--log-level", "4"])
        .current_dir("../web")
        .output()?;
    let build_out = String::from_utf8(build_output.stdout)?;
    println!("{}", build_out);
    println!("{}", String::from_utf8(build_output.stderr)?);
    assert!(build_output.status.success());

    let artifacts: Vec<PathBuf> = build_out
        .split('\n')
        .filter_map(|line| line.split_whitespace().nth(1))
        .filter(|token| token.starts_with("dist/"))
        .map(FromStr::from_str)
        .collect::<Result<_, _>>()?;
    assert!(!artifacts.is_empty());

    // Extracting the sources from the output is tricky
    let sources: Vec<PathBuf> = [
        "../web/package.json",
        "../web/package.lock",
        "../web/index.html",
        "../web/js",
        "../web/crate/Cargo.toml",
        "../web/crate/Cargo.lock",
        "../webcrate/src",
    ]
    .iter()
    .copied()
    .map(FromStr::from_str)
    .collect::<Result<_, _>>()?;

    Ok((sources, artifacts))
}

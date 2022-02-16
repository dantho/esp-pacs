// TODO: add some logging/output of some sort

use std::{
    env,
    fs::{self, File},
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
};

use anyhow::{Error, Result};
use clap::Parser;
use form::util::create_directory_structure;
use strum::VariantNames;
use strum_macros::{Display, EnumString, EnumVariantNames};
use svd2rust::{generate::device::render, load_from, util::build_rs, Config, Target};
use svdtools::patch::process_file;
use toml::Value;

#[derive(Debug, Display, EnumString, EnumVariantNames)]
pub enum Chip {
    #[strum(serialize = "esp32")]
    Esp32,
    #[strum(serialize = "esp32c3")]
    Esp32c3,
    #[strum(serialize = "esp32s2")]
    Esp32s2,
    #[strum(serialize = "esp32s3")]
    Esp32s3,
    #[strum(serialize = "esp8266")]
    Esp8266,
}

#[derive(Debug, Parser)]
struct Opts {
    /// Chip to target
    #[clap(possible_values = Chip::VARIANTS)]
    chip: Chip,
    /// Only patch the SVD, do not generate or build the PAC
    #[clap(long, conflicts_with = "generate-only")]
    patch_only: bool,
    /// Patch the SVD and generate the PAC, but do not build it
    #[clap(long, conflicts_with = "patch-only")]
    generate_only: bool,
}

fn main() -> Result<()> {
    let opts = Opts::parse();

    let chip = opts.chip.to_string();
    let chip = chip.as_str();

    // The directory containing the cargo manifest for the 'xtask' package is a
    // subdirectory within the cargo workspace.
    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace = workspace.parent().unwrap().canonicalize()?;

    // Remove the src/ directory before we begin. If this fails we will assume it's
    // because the directory does not exist.
    let path = workspace.join(chip);
    fs::remove_dir_all(&path.join("src")).ok();

    // Always patch the SVD upon execution. If '--patch-only' has NOT been set,
    // then additionally generate and format the PAC. If '--generate-only' has NOT
    // been set, clean and build the PAC crate.
    patch_svd(chip, &path)?;
    if !opts.patch_only {
        generate_pac(chip, &path)?;
        format(&path)?;
        if !opts.generate_only {
            clean(&path)?;
            build(&path)?;
        }
    }

    Ok(())
}

fn patch_svd(chip: &str, path: &PathBuf) -> Result<()> {
    let svd_path = path.join("svd");

    let yaml_file = svd_path.join("patches").join(format!("{chip}.yaml"));
    process_file(&yaml_file)?;

    let from = svd_path.join(format!("{chip}.base.svd.patched"));
    let to = svd_path.join(format!("{chip}.svd"));
    fs::rename(from, to)?;

    Ok(())
}

fn generate_pac(chip: &str, path: &PathBuf) -> Result<()> {
    let svd_file = path.join("svd").join(format!("{chip}.svd"));
    let input = fs::read_to_string(svd_file)?;

    let mut config = Config::default();
    config.target = get_svd2rust_target(path)?;
    config.output_dir = path.clone();

    let device = load_from(&input, &config)?;

    let mut device_x = String::new();
    let items = render(&device, &config, &mut device_x)?;
    let data = items.to_string().replace("]", "]\n");

    let mut file = File::create(path.join("lib.rs"))?;
    file.write_all(data.as_ref())?;

    writeln!(File::create(path.join("device.x"))?, "{}", device_x)?;
    writeln!(File::create(path.join("build.rs"))?, "{}", build_rs())?;

    Ok(())
}

fn format(path: &PathBuf) -> Result<()> {
    let lib_file = path.join("lib.rs");

    let base_dir = path.join("src");
    let string_contents = fs::read_to_string(&lib_file)?;
    create_directory_structure(base_dir, string_contents).map_err(Error::msg)?;

    fs::remove_file(&lib_file)?;

    // TODO: consider using 'rustfmt' directly, as a library
    Command::new("cargo")
        .arg("fmt")
        .current_dir(path)
        .output()?;

    Ok(())
}

fn clean(path: &PathBuf) -> Result<()> {
    Command::new("cargo")
        .arg("clean")
        .current_dir(path)
        .output()?;

    Ok(())
}

fn build(path: &PathBuf) -> Result<()> {
    let channel = get_release_channel(path)?;
    let target = get_build_target(path)?;

    Command::new("cargo")
        .args(&[
            &format!("+{channel}"),
            "build",
            "-Z",
            "build-std=core",
            "--target",
            &target,
        ])
        .current_dir(path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    Ok(())
}

fn get_release_channel(path: &PathBuf) -> Result<String> {
    let toolchain_file = path.join("rust-toolchain.toml");
    let channel = extract_toml_value(&toolchain_file, &["toolchain", "channel"])?;

    Ok(channel)
}

fn get_build_target(path: &PathBuf) -> Result<String> {
    let config_file = path.join(".cargo").join("config.toml");
    let target = extract_toml_value(&config_file, &["build", "target"])?;

    Ok(target)
}

fn get_svd2rust_target(path: &PathBuf) -> Result<Target> {
    if get_build_target(path)?.contains("riscv") {
        Ok(Target::RISCV)
    } else {
        Ok(Target::XtensaLX)
    }
}

fn extract_toml_value(path: &PathBuf, keys: &[&str]) -> Result<String> {
    let contents = fs::read_to_string(path)?;
    let value = contents.parse::<Value>()?;

    let mut item = &value;
    for key in keys {
        item = item.get(key).unwrap();
    }

    let item = item.to_string().replace("\"", "");

    Ok(item)
}

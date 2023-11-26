use rsh_test_support::rsh;
use pretty_assertions::assert_eq;
use std::fs;
use std::path::Path;

#[cfg(not(target_os = "windows"))]
fn adjust_canonicalization<P: AsRef<Path>>(p: P) -> String {
    p.as_ref().display().to_string()
}

#[cfg(target_os = "windows")]
fn adjust_canonicalization<P: AsRef<Path>>(p: P) -> String {
    const VERBATIM_PREFIX: &str = r"\\?\";
    let p = p.as_ref().display().to_string();
    if let Some(stripped) = p.strip_prefix(VERBATIM_PREFIX) {
        stripped.to_string()
    } else {
        p
    }
}

#[test]
fn test_default_config_path() {
    let config_dir = rsh_path::config_dir().expect("Could not get config directory");
    let config_dir_rsh = config_dir.join("rsh");
    // Create the config dir folder structure if it does not already exist
    if !config_dir_rsh.exists() {
        let _ = fs::create_dir_all(&config_dir_rsh);
    }
    let cwd = std::env::current_dir().expect("Could not get current working directory");
    let config_path = config_dir_rsh.join("config.rsh");

    // Create an empty file for canonicalization if it doesn't already exist
    if !config_path.exists() {
        let _ = std::fs::File::create(&config_path);
    }

    // We use canonicalize here in case the config or env is symlinked since $rsh.config-path is returning the canonicalized path in #8653
    let canon_config_path = adjust_canonicalization(
        std::fs::canonicalize(config_path).expect("canonicalize config-path failed"),
    );

    let actual = rsh!(cwd: &cwd, "$rsh.config-path");
    assert_eq!(actual.out, canon_config_path);
    let env_path = config_dir_rsh.join("env.rsh");

    // Create an empty file for canonicalization if it doesn't already exist
    if !env_path.exists() {
        let _ = std::fs::File::create(&env_path);
    }

    let canon_env_path = adjust_canonicalization(
        std::fs::canonicalize(env_path).expect("canonicalize of env-path failed"),
    );
    let actual = rsh!(cwd: &cwd, "$rsh.env-path");
    assert_eq!(actual.out, canon_env_path);
}

#[test]
fn test_alternate_config_path() {
    let config_file = "crates/rsh-utils/src/sample_config/default_config.rsh";
    let env_file = "crates/rsh-utils/src/sample_config/default_env.rsh";

    let cwd = std::env::current_dir().expect("Could not get current working directory");

    let config_path =
        rsh_path::canonicalize_with(config_file, &cwd).expect("Could not get config path");
    let actual = rsh!(
        cwd: &cwd,
        format!("rsh --config {config_path:?} -c '$rsh.config-path'")
    );
    assert_eq!(actual.out, config_path.to_string_lossy().to_string());

    let env_path = rsh_path::canonicalize_with(env_file, &cwd).expect("Could not get env path");
    let actual = rsh!(
        cwd: &cwd,
        format!("rsh --env-config {env_path:?} -c '$rsh.env-path'")
    );
    assert_eq!(actual.out, env_path.to_string_lossy().to_string());
}

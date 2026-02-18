//! End-to-end CLI tests using trycmd
//!
//! These are snapshot tests that verify CLI behavior matches expected output.
//! Similar to Cypress for web apps, but for CLI tools.

#[cfg(test)]
mod tests {
    use trycmd::TestCases;

    #[test]
    fn cli_help() {
        TestCases::new()
            .case("tests/cli/help.toml");
    }

    #[test]
    fn cli_version() {
        TestCases::new()
            .case("tests/cli/version.toml");
    }

    #[test]
    fn cli_init_help() {
        TestCases::new()
            .case("tests/cli/init_help.toml");
    }

    #[test]
    fn cli_init_noninteractive() {
        TestCases::new()
            .case("tests/cli/init_noninteractive.toml");
    }

    #[test]
    fn cli_cut_help() {
        TestCases::new()
            .case("tests/cli/cut_help.toml");
    }

    #[test]
    fn cli_carve_help() {
        TestCases::new()
            .case("tests/cli/carve_help.toml");
    }

    #[test]
    fn cli_chamfer_help() {
        TestCases::new()
            .case("tests/cli/chamfer_help.toml");
    }

    #[test]
    fn cli_check_help() {
        TestCases::new()
            .case("tests/cli/check_help.toml");
    }

    #[test]
    fn cli_taint_help() {
        TestCases::new()
            .case("tests/cli/taint_help.toml");
    }

    #[test]
    fn cli_status() {
        TestCases::new()
            .case("tests/cli/status_help.toml");
    }

    #[test]
    fn cli_taint_status() {
        TestCases::new()
            .case("tests/cli/taint_status.toml");
    }

    #[test]
    fn cli_robot_flag() {
        TestCases::new()
            .case("tests/cli/robot_flag.toml");
    }

    #[test]
    fn cli_cut_file() {
        TestCases::new()
            .case("tests/cli/cut_file.toml");
    }

    /// Golden oracle tests - byte-for-byte output comparison
    #[test]
    fn golden_oracles() {
        TestCases::new()
            .case("tests/cli/golden/help.trycmd/help.toml");
    }
}

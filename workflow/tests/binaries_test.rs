use std::env;
use std::process::Command;


fn run_binary(bin_name: &str, options: Option<&str>) -> bool {
    let mut cmd = Command::new("cargo");
    cmd.args(&["run", "--bin", bin_name]);

    if let Some(opts) = options {
        cmd.arg("--").args(opts.split_whitespace());
    }

    println!("{:?}", &cmd);

    let status = cmd
        .status()
        .expect(&format!("Failed to execute binary '{}'", bin_name));

    status.success()
}

#[test]
fn test_binaries() {
    let binaries = vec![
        ("parse", None), 
        ("plot", Some("-w tb_pcr")), 
        ("executor", None), 
        ("schemagen", None)
    ];

    for (bin_name, options) in binaries {
        assert!(
            run_binary(bin_name, options),
            "Binary '{}' did not run to completion without erroring out",
            bin_name
        );
    }
}

use std::process::Command;

#[test]
fn help_documents_non_interactive_commands() {
    let output = Command::new(env!("CARGO_BIN_EXE_arazzo-proof"))
        .arg("--help")
        .output()
        .unwrap();
    assert!(output.status.success());
    let help = String::from_utf8(output.stdout).unwrap();
    assert!(help.contains("run"));
    assert!(help.contains("compare"));
    assert!(help.contains("Run an Arazzo workflow and keep the proof"));
}

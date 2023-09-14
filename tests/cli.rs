#[cfg(test)]
mod tests {
    use assert_cmd::prelude::*;
    use assert_fs::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    #[test]
    pub fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("rust_grrs")?;

        cmd.arg("foobar").arg("test/file/doesnt/exist");

        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("could not read file"));

        Ok(())
    }

    #[test]
    pub fn happy_path() -> Result<(), Box<dyn std::error::Error>> {
        let file = assert_fs::NamedTempFile::new("sample.txt")?;
        file.write_str("A test\nB test\nMy cool pattern")?;

        let mut cmd = Command::cargo_bin("rust_grrs")?;

        cmd.arg("cool").arg(file.path());

        cmd.assert()
            .success()
            .stdout(predicate::str::contains("My cool pattern"));

        Ok(())
    }
}

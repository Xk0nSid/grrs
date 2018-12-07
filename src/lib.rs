use exitfailure::ExitFailure;
use failure::ResultExt;

pub fn find_matches(
    content: &str,
    pattern: &str,
    writer: &mut std::io::Write,
) -> Result<(), ExitFailure> {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line)
                .with_context(|_| format!("failed to write to provied writer"))?;
        }
    }
    Ok(())
}
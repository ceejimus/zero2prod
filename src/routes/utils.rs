pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "[_] {}\n", e)?;
    let chain = std::iter::successors(e.source(), |e| e.source());
    for (i, link) in chain.enumerate() {
        writeln!(f, "[{i}] Caused by:\n\t{}", link)?;
    }
    Ok(())
}

mod search;

use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let target = args[1]
        .parse::<u8>()
        .with_context(|| format!("target: ({}) value is not an u8 value", args[1]))?;

    let path: String = "sorted_ints.txt".into();
    let metadata =
        std::fs::metadata(&path).with_context(|| format!("read metadata of file {}", &path))?;

    let _ = search::binary_search::binary_search(target, &path, &metadata)?;
    Ok(())
}

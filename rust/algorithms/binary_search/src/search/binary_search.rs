use anyhow::Context;
use std::io::{Read, Seek};

pub(crate) fn binary_search(
    target: u8,
    path: &str,
    metadata: &std::fs::Metadata,
) -> anyhow::Result<()> {
    let mut hi = metadata.len() - 1;
    let mut lo = 0;
    let mut mid = 0;

    while lo <= hi {
        mid = lo + (hi - lo) / 2;

        let mut fd = std::fs::File::open(path).with_context(|| format!("open file {}", path))?;
        fd.seek(std::io::SeekFrom::Start(mid - 1))
            .with_context(|| format!("seek position {} in file {}", mid - 1, &path))?;

        let mut buffer = [0; 1];
        let _ = fd
            .read_exact(&mut buffer)
            .context("reading single byte after seek from file")?;

        let value = buffer[0] - 48;

        if value == 10 {
            break;
        }

        if value == target {
            println!("Found target {} in file {}", target, path);
            return Ok(());
        }

        if target < value {
            hi = mid - 1
        }

        if target > value {
            lo = mid + 1
        }
    }
    println!("Not found target {} in file {}", target, path);
    Ok(())
}

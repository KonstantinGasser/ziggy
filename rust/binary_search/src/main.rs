use std::io::Seek;

use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let path: String = "sorted_ints.txt".into();

    let metadata =
        std::fs::metadata(&path).with_context(|| format!("read metadata of file {}", &path))?;

    let mut hi = metadata.len() - 1;
    let mut lo = 0;
    let mut mid = 0;

    let target = 4;
    while lo <= hi {
        mid = lo + (hi - lo) / 2;

        let value = match std::fs::File::open(&path) {
            Ok(mut file) => match file.seek(std::io::SeekFrom::Start((mid - 1) as u64)) {
                Ok(value) => value,
                Err(err) => panic!("unable to seek pos: {:?}: {:?}", mid - 1, err),
            },
            Err(err) => panic!("unable to open file {:?}: {:?}", &path, err),
        };

        if value == 10 {
            break;
        }

        if value == target {
            println!("Found target {} in file {}", target, path);
            break;
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

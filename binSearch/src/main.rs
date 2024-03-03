use std::io::Seek;

fn main() {
    let path: String = "sorted_ints.txt".into();

    let bytes = match std::fs::read(&path) {
        Ok(bytes) => bytes,
        Err(err) => panic!("unable to read file {:?} into Vec<u8>: {:?}", &path, err),
    };

    let mut hi = bytes.len() - 1;
    let mut lo = 0;
    let mut mid = 0;

    let target = 4;
    while lo <= hi {
        mid = lo + (hi - lo) / 2;

        let value = std::fs::File::open(&path)
            .unwrap()
            .seek(std::io::SeekFrom::Start(((mid - 1) as u64)))
            .unwrap();

        if value == 10 {
            break;
        }

        if value == target {
            println!("Found target {} in file {}", target, path);
            return;
        }

        if target < value {
            hi = mid - 1
        }

        if target > value {
            lo = mid + 1
        }
    }
    println!("Not found target {} in file {}", target, path);
}

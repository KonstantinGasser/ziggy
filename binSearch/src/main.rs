fn main() {
    let path: String = "sorted_ints.txt".into();

    let bytes = match std::fs::read(&path) {
        Ok(bytes) => bytes,
        Err(err) => panic!("unable to read file {:?} into Vec<u8>: {:?}", &path, err),
    };

    let mut hi = bytes.len() - 1;
    let mut lo = 0;
    let mut mid = 0;

    let target = 5;
    while lo <= hi {
        mid = lo + (hi - lo) / 2;

        let mut value = usize::from(bytes[mid]);
        if value == 10 {
            break;
        }

        value -= 48;

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

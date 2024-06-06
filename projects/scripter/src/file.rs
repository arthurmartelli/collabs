use std::{fs::File, io::BufReader};

// 10 MB
const CHUNK_SIZE: usize = 10 * 1024 * 1024;

pub fn get_file_reader<T: Into<String>>(file_name: T) -> BufReader<File> {
    let file_name = file_name.into();
    // should crash if we can't open the file
    let file = File::open(&file_name).unwrap_or_else(|_| panic!("Unable to open {}", &file_name));
    BufReader::with_capacity(CHUNK_SIZE, file)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, io::BufRead};

    fn test_file<T: Into<String>>(file_name: T, contents: T) {
        let file_name = file_name.into();
        let contents = contents.into();

        fs::write(&file_name, &contents).expect("Unable to write to test file");

        // assert chunk size
        let reader = get_file_reader(file_name.to_string());
        assert_eq!(reader.capacity(), CHUNK_SIZE);

        // assert file contents
        let mut lines = reader.lines();
        for line in contents.lines() {
            let correct_line = line.to_string();
            let file_line = lines.next().unwrap().unwrap();
            assert_eq!(correct_line, file_line);
        }

        fs::remove_file(file_name).expect("Unable to remove test file");
    }

    #[test]
    fn test_small_file_read() {
        test_file(
            "test_small_file_read.auto",
            "This is a test file.\nWith multiple lines!",
        );
    }

    #[test]
    fn test_large_file_read() {
        test_file(
            "test_large_file_read.auto",
            "This is a test file.\nWith multiple lines.\n"
                .repeat(10e5 as usize) // around 40mb file size
                .as_str(),
        );
    }
}

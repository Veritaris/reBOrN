use std::fs;
use std::io::BufReader;
use std::path::Path;

pub fn seconds_to_minutes_str(seconds: u32) -> String {
    let minutes: f64 = seconds as f64 / 60.0;
    let seconds_partial: f64 = (seconds as f64 - minutes * 60.0) / 60.0;
    return format!("{:.2}", minutes + seconds_partial);
}

pub fn bool_to_str(val: bool) -> String {
    return String::from(if val { "1" } else { "0" });
}

fn traverse_jar(path: &Path) {
    for i in path.read_dir().unwrap() {
        match i {
            Ok(ref res) => {
                if res.file_type().unwrap().is_dir() {
                    traverse_jar(path.join(res.file_name()).as_path());
                } else {
                    let file = fs::File::open(res.path().as_path()).unwrap();
                    let reader = BufReader::new(file);
                    match classy::read_class(reader) {
                        Ok(cls) => {
                            println!("{:?} {:?}", res.file_name(), cls.this_class);
                        }
                        Err(err) => {
                            println!("{:?} {}", res.file_name(), err.to_string())
                        }
                    }
                }
            }
            Err(_) => {}
        }
    }
}
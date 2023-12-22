
use std::io::Write;

pub fn _input(prompt: &str) -> Result<String, String> {
    let mut buffer = String::new();

    print!("{}", prompt);
    std::io::stdout()
        .flush()
        .expect("unable to flush stdout");

    return match std::io::stdin().read_line(&mut buffer) {
        Ok(_) => Ok(String::from(&buffer[0..buffer.len()-1])),
        Err(e) => Err(e.to_string()),
    };

}

pub fn _write_to_ron_file<
    P: AsRef<std::path::Path>,
    T: serde::Serialize,
>(fpath: P, object: T) {
    std::fs::write(
        fpath,
        ron::ser::to_string(&object).unwrap()
    ).unwrap();
}

pub fn read_from_ron_file <
    P: AsRef<std::path::Path>,
    T: for<'a> serde::Deserialize<'a> + Clone + Default,
> (fpath: P) -> Result<T, Box<dyn std::error::Error>> {
    return match std::fs::read_to_string(fpath) {
        Ok(string) => {
            let string: std::rc::Rc<str> = string.as_str().into();
            match ron::de::from_str::<T>(string.as_ref()) {
                Ok(object) => Ok(object.clone()),
                Err(e) => Err(Box::new(e)),
            }
        },
        Err(_) => Ok(T::default())
    }
}


#[derive(Clone)]
pub struct TimeWindow {
    pub start: chrono::DateTime<chrono::Utc>,
    pub end: chrono::DateTime<chrono::Utc>
}




use std::time::SystemTime;

pub fn time(name: &str, function_to_time: impl Fn()) {
    let time = SystemTime::now();
    function_to_time();
    time.elapsed()
        .iter()
        .for_each(|duration| println!("{name} took {duration:?} to run"));
}

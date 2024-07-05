#[macro_export]
macro_rules! time {
    () => {
        println!()
    };
    ($val:expr $(,)?) => {
        {
            print!("{}", stringify!($val));
            let snap = std::time::Instant::now();
            let tmp = { $val };
            println!(" -> took: {}ms", snap.elapsed().as_nanos() as f64 / 1000000.);
            tmp
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($(result!($val)),+,)
    }
}
// You can use the `use` keyword to bring module paths from modules from
// anywhere and especially from the standard library into your scope.

// TODO: Bring `SystemTime` and `UNIX_EPOCH` from the `std::time` module into
// your scope. Bonus style points if you can do it with one line!
// use ???;

fn main() {
    use std::time::SystemTime as sysTime;
    use std::time::UNIX_EPOCH as unixEpoch;
    match sysTime::now().duration_since(unixEpoch) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

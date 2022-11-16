use md5;

fn main() {
    let salt = String::from("ckczppom");
    let mut count = 0;

    let result = loop {
        count += 1;
        let digest = md5::compute(format!("{}{}", salt, count));
        //println!("{} {:x}", count, digest);
        if format!("{:x}", digest).starts_with("000000") {
            break count;
        }
    };

    println!("{}", result);
}

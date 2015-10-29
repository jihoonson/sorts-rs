extern crate gcc;

fn main() {
    gcc::Config::new().file("src/shift.c").compile("libshift.a");
}


mod measurements;

fn run(d : &Vec<u32>) {
    println!("Increases={}", d.iter().enumerate()
        .skip(1)
        .filter(|x| &d[x.0-1]  < x.1)
        .count());
    println!("3 window increases={}", d.iter().enumerate()
        .skip(3)
        .filter(|x| (&d[x.0-3] + &d[x.0-2] + &d[x.0-1])  < (&d[x.0-2] + &d[x.0-1] + &d[x.0]))
        .count());
}

fn main() {
    println!("==== Test Data ====");
    run(&measurements::get_test());
    println!("==== Real Data ====");
    run(&measurements::get());
}

use ddmonitor::handlers;

fn main() {
    let a: handlers::ActionInfo = handlers::ActionInfo::Hello;
    let x = a.wrapper();
    println!("x: {:?}", &x);

    let b = handlers::ActionInfo::unwrap(&x);
    println!("b: {:?}", &b);
}

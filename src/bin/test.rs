use {ddmonitor::handlers, env_logger::Env};

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let a: handlers::ActionInfo = handlers::ActionInfo::ActionSample(1, 2);
    let x = a.wrapper();
    println!("x: {:?}", &x);

    let b = handlers::ActionInfo::unwrap(&x);
    println!("b: {:?}", &b);
    b.do_action();
}

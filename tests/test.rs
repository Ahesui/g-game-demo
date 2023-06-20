use gtest::{Log, Program, System};
use hello_world::TmgAction;
use hello_world::TmgEvent;

#[test]
fn hello_test() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);
    let res = program.send(2, String::from("Tamagotchi Name"));
    println!("Result: {:?}", res);
    assert!(!res.main_failed());
    assert!(!res.others_failed());

    
    // test `Name` action
    let res = program.send(2, TmgAction::Name);
    let expected_log = Log::builder()
        .dest(2)
        .payload(TmgEvent::Name(String::from("Tamagotchi Name")));
    println!("Expected log: {:?}", expected_log);
    println!("Result: {:?}", res);
    assert!(res.contains(&expected_log));

    // test `Age` action
    let res = program.send(2, TmgAction::Age);
    let expected_age = 0; // Replace with the actual age calculation based on birth date and current timestamp
    let expected_log = Log::builder()
        .dest(2)
        .payload(TmgEvent::Age(expected_age));
    println!("Expected log: {:?}", expected_log);
    println!("Result: {:?}", res);
    assert!(res.contains(&expected_log));
}

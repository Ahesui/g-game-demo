use gstd::prelude::*;
use gtest::{Log, Program, System};
use game_io::{TmgAction, TmgEvent};
const OWNER: u64 = 100;
const INIT_MESSAGE: &str = "Test Tamagotchi";

fn init_tamagotchi(sys: &System) {
    sys.init_logger();
    let tamagotchi = Program::current(sys);
    tamagotchi.send(OWNER,INIT_MESSAGE.to_string());
}

const TAMAGOTCHI_ID: u64 = 1;

#[test]
fn name() {
    let sys = System::new();
    init_tamagotchi(&sys);

    let tamagotchi = sys.get_program(TAMAGOTCHI_ID);

    let res = tamagotchi.send(OWNER,TmgAction::Name);
    let log = Log::builder().dest(OWNER).payload(TmgEvent::Name(INIT_MESSAGE.to_string()));
    assert!(res.contains(&log));
}

#[test]
fn age() {
    let sys = System::new();
    init_tamagotchi(&sys);

    let tamagotchi = sys.get_program(TAMAGOTCHI_ID);

    let res = tamagotchi.send(OWNER,TmgAction::Age);
    let log = Log::builder().dest(OWNER).payload(TmgEvent::Age(0)); // Update the expected age value based on your logic
    assert!(res.contains(&log));
}

#[test]
fn feed() {
    let sys = System::new();
    init_tamagotchi(&sys);

    let tamagotchi = sys.get_program(TAMAGOTCHI_ID);

    let res = tamagotchi.send(OWNER,TmgAction::Feed);
    let log = Log::builder().dest(OWNER).payload(TmgEvent::Fed);
    assert!(res.contains(&log));
}

#[test]
fn play() {
    let sys = System::new();
    init_tamagotchi(&sys);

    let tamagotchi = sys.get_program(TAMAGOTCHI_ID);

    let res = tamagotchi.send(OWNER,TmgAction::Play);
    let log = Log::builder().dest(OWNER).payload(TmgEvent::Entertained);
    assert!(res.contains(&log));
}

#[test]
fn sleep() {
    let sys = System::new();
    init_tamagotchi(&sys);

    let tamagotchi = sys.get_program(TAMAGOTCHI_ID);

    let res = tamagotchi.send(OWNER,TmgAction::Sleep);
    let log = Log::builder().dest(OWNER).payload(TmgEvent::Slept);
    assert!(res.contains(&log));
}


#[test]
fn transfer() {
    let sys = System::new();
    init_tamagotchi(&sys);

    let tamagotchi = sys.get_program(TAMAGOTCHI_ID);
    let new_owner = 200;

    let res = tamagotchi.send(OWNER, TmgAction::Transfer(new_owner.into()));
    let log1 = Log::builder().dest(OWNER).payload(TmgEvent::Transfer(new_owner.into()));
    assert!(res.contains(&log1));

    let res2 = tamagotchi.send(new_owner, TmgAction::Name);
    let log2 = Log::builder().dest(new_owner).payload(TmgEvent::Name(INIT_MESSAGE.to_string()));
    assert!(res2.contains(&log2));
}

#[test]
fn approve() {
    let sys = System::new();
    init_tamagotchi(&sys);

    let tamagotchi = sys.get_program(TAMAGOTCHI_ID);
    let allowed_account = 300;

    let res = tamagotchi.send(OWNER, TmgAction::Approve(allowed_account.into()));
    let log = Log::builder().dest(OWNER).payload(TmgEvent::Approve(allowed_account.into()));
    assert!(res.contains(&log));

    let res2 = tamagotchi.send(allowed_account, TmgAction::Feed);
    let log2 = Log::builder().dest(allowed_account).payload(TmgEvent::Fed);
    assert!(res2.contains(&log2));
}

#[test]
fn revoke_approval() {
    let sys = System::new();
    init_tamagotchi(&sys);

    let tamagotchi = sys.get_program(TAMAGOTCHI_ID);
    let allowed_account = 300;

    let _ = tamagotchi.send(OWNER, TmgAction::Approve(allowed_account.into()));

    let res = tamagotchi.send(OWNER, TmgAction::RevokeApproval);
    let log = Log::builder().dest(OWNER).payload(TmgEvent::RevokeApproval);
    assert!(res.contains(&log));

    let res2 = tamagotchi.send(allowed_account, TmgAction::Feed);
    assert!(!res2.main_failed());
}

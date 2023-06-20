#![no_std]
use gstd::{msg, prelude::*, debug, exec};

#[derive(Encode, Decode, TypeInfo)]
pub enum TmgAction {
    Name,
    Age,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum TmgEvent {
    Name(String),
    Age(u64),
}

#[derive(Default, Encode, Decode, TypeInfo)]
pub struct Tamagotchi {
    pub name: String,
    pub date_of_birth: u64,
}

static mut TAMAGOTCHI: Option<Tamagotchi> = None;

#[no_mangle]
extern "C" fn metahash() {
    let metahash: [u8; 32] = include!("../.metahash");
    msg::reply(metahash, 0).expect("Failed to share metahash");
}

#[no_mangle]
extern "C" fn state() {
    let tamagotchi = unsafe {
        TAMAGOTCHI
            .as_ref()
            .expect("The contract is not initialized")
    };
    msg::reply(tamagotchi, 0).expect("Failed to share state");
}

#[no_mangle]
extern "C" fn handle() {
    let input_message: TmgAction = msg::load().expect("Error in loading TmgAction");
    let tamagotchi = unsafe {
        TAMAGOTCHI
            .as_mut()
            .expect("The contract is not initialized")
    };

    match input_message {
        TmgAction::Name => {
            debug!("Message: Name");
            msg::reply(TmgEvent::Name(tamagotchi.name.clone()), 0)
                .expect("Error in sending reply");
        }
        TmgAction::Age => {
            debug!("Message: Age");
            let current_timestamp =  exec::block_timestamp();
            
            let age = current_timestamp - tamagotchi.date_of_birth;
            msg::reply(TmgEvent::Age(age), 0)
                .expect("Error in sending reply");
        }
    }
}

#[no_mangle]
extern "C" fn init() {
    let init_message: String = msg::load().expect("Can't load init message");
    let current_timestamp =  exec::block_timestamp();
    

    debug!(
        "Program was initialized with message {:?} and timestamp {}",
        init_message, current_timestamp
    );

    unsafe {
        TAMAGOTCHI = Some(Tamagotchi {
            name: init_message,
            date_of_birth: current_timestamp,
        });
    }

    msg::reply((), 0).expect("Failed to send initialization reply");
}

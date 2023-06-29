#![no_std]
use gstd::{msg, prelude::*, debug, exec,ActorId};
use game_io::{TmgAction, TmgEvent};
#[derive(Default, Encode, Decode, TypeInfo)]
pub struct Tamagotchi {
    pub name: String,
    pub date_of_birth: u64,
    pub owner: ActorId,
    pub fed: u64,
    pub fed_block: u64,
    pub entertained: u64,
    pub entertained_block: u64,
    pub rested: u64,
    pub rested_block: u64,
    pub allowed_account: Option<ActorId>
}
const HUNGER_PER_BLOCK :u64 = 1; //: how much Tamagotchi becomes hungry for the block ;
const ENERGY_PER_BLOCK :u64 = 2; // - how much Tamagotchi loses energy per block;
const BOREDOM_PER_BLOCK :u64 = 2; // - how bored Tamagotchi gets per block;
const FILL_PER_SLEEP :u64 = 1000; // - how much energy Tamagotchi gets per sleep;
const FILL_PER_FEED :u64 = 1000; // - how much Tamagotchi becomes full during feeding;
const FILL_PER_ENTERTAINMENT :u64 = 1000;  // - how much Tamagotchi becomes happy during feeding;

static mut TAMAGOTCHI: Option<Tamagotchi> = None;

impl Tamagotchi {
    
    fn calculate_age(&self) -> u64 {
        let _current_timestamp =  exec::block_timestamp();
        let seconds_per_year: u64 = 365 * 24 * 3600;
        (_current_timestamp - self.date_of_birth)/(seconds_per_year*1000) as u64
    }

    fn feed(&mut self) {      
        let _current_timestamp =  exec::block_timestamp();
        let _current_block = exec::block_height() as u64;
        let mut hungry_val = (_current_block - self.fed_block)*HUNGER_PER_BLOCK;
        hungry_val =  if hungry_val>0 {hungry_val } else { 0 }; 
        self.fed = hungry_val + 1;
        self.fed_block = _current_block;
    }

    fn play(&mut self) {
        let _current_timestamp =  exec::block_timestamp();
        let _current_block = exec::block_height() as u64;
        let mut happy_val = (_current_block - self.entertained_block)*BOREDOM_PER_BLOCK;
        happy_val =  if happy_val>0 { happy_val } else { 0 }; 
        self.entertained = happy_val + 1;
        self.entertained_block = _current_block;
    }

    fn sleep(&mut self) {
        let _current_timestamp =  exec::block_timestamp();
        let _current_block = exec::block_height() as u64;
        let mut energy_val = (_current_block - self.rested_block)*ENERGY_PER_BLOCK;
        energy_val =  if energy_val>0 { energy_val } else { 0 }; 
        self.rested = energy_val + 1;
        self.rested_block = _current_block;
    }

}


#[no_mangle]
extern "C" fn init() {
    let init_message: String = msg::load().expect("Can't load init message");
    let _current_timestamp =  exec::block_timestamp();
    let _current_block = exec::block_height() as u64;

    debug!(
        "Program was initialized with message {:?} and timestamp {}",
        init_message, _current_timestamp
    );

    unsafe {
        TAMAGOTCHI = Some(Tamagotchi {
            name: init_message,
            date_of_birth: _current_timestamp,
            owner: msg::source(),
            fed: FILL_PER_FEED,
            fed_block: _current_block,
            entertained: FILL_PER_ENTERTAINMENT,
            entertained_block: _current_block,
            rested: FILL_PER_SLEEP,
            rested_block: _current_block,
            allowed_account: Some(msg::source()),
        });
    }

    msg::reply((), 0).expect("Failed to send initialization reply");
}

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
            .expect("The contract is not initialized");
    };
    msg::reply(tamagotchi, 0).expect("Failed to share state");

      

}

#[no_mangle]
extern "C" fn handle() {


    let action: TmgAction = msg::load().expect("Unable to decode `TmgAction`");
    let tamagotchi = unsafe { TAMAGOTCHI.as_mut().expect("Program hasn't been initialized") };
    match action {
        TmgAction::Name => {
            msg::reply(TmgEvent::Name(tamagotchi.name.clone()), 0)
                .expect("Error in reply `TmgEvent::Name`");
        }
        TmgAction::Age => {
            let age = tamagotchi.calculate_age();
            msg::reply(TmgEvent::Age(age), 0)
                .expect("Error in reply `TmgEvent::Age`");
        }
        TmgAction::Feed => {
            tamagotchi.feed();
            msg::reply(TmgEvent::Fed, 0).expect("Error in reply `TmgEvent::Fed`");
        }
        TmgAction::Play => {
            tamagotchi.play();
            msg::reply(TmgEvent::Entertained, 0)
                .expect("Error in reply `TmgEvent::Entertained`");
        }
        TmgAction::Sleep => {
            tamagotchi.sleep();
            msg::reply(TmgEvent::Slept, 0).expect("Error in reply `TmgEvent::Slept`");
        }
        TmgAction::Transfer(new_owner) => {
            tamagotchi.owner = new_owner;
            msg::reply(TmgEvent::Transfer(new_owner), 0)
                .expect("Error in reply `TmgEvent::Transfer`");
        }
        TmgAction::Approve(allowed_account) => {
            tamagotchi.allowed_account = Some(allowed_account);
            msg::reply(TmgEvent::Approve(allowed_account), 0)
                .expect("Error in reply `TmgEvent::Approve`");
        }
        TmgAction::RevokeApproval => {
            tamagotchi.allowed_account = None;
            msg::reply(TmgEvent::RevokeApproval, 0)
                .expect("Error in reply `TmgEvent::RevokeApproval`");
        }
    }
}



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
    fn get_current_feed(&mut self) -> u64  {
        let _current_block = exec::block_height() as u64;
        let  hungry_val = (_current_block - self.fed_block)*HUNGER_PER_BLOCK;
        self.fed  =  if self.fed > hungry_val {self.fed -hungry_val } else { 0 };
        self.fed
    }
    fn get_current_entertained(&mut self) -> u64  {
        let _current_block = exec::block_height() as u64;
        let borring_val = (_current_block - self.entertained_block)*BOREDOM_PER_BLOCK;
        self.entertained =  if self.entertained > borring_val { self.entertained-borring_val } else { 0 }; 
        self.entertained
    }
    fn get_current_rested(&mut self) -> u64  {
        let _current_block = exec::block_height() as u64;
        let energy_val = (_current_block - self.rested_block)*ENERGY_PER_BLOCK;
        self.rested =  if self.rested > energy_val { self.rested-energy_val } else { 0 }; 
        self.rested
    }
    fn calculate_age(&self) -> u64 {
        let _current_timestamp =  exec::block_timestamp();
        let seconds_per_year: u64 = 365 * 24 * 3600;
        (_current_timestamp - self.date_of_birth)/(seconds_per_year*1000) as u64
    }

    fn feed(&mut self) {      
        self.fed  = self.get_current_feed();
        self.fed += FILL_PER_FEED;
        self.fed_block =  exec::block_height() as u64;
    }

    fn play(&mut self) {
        self.entertained = self.get_current_entertained();
        self.entertained += FILL_PER_ENTERTAINMENT;
        self.entertained_block = exec::block_height() as u64;
    }

    fn sleep(&mut self) {
        self.rested = self.get_current_rested(); 
        self.rested += FILL_PER_SLEEP;
        self.rested_block = exec::block_height() as u64;
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
}

#[no_mangle]
extern "C" fn metahash() {
    let metahash: [u8; 32] = include!("../.metahash");
    msg::reply(metahash, 0).expect("Failed to share metahash");
}

#[no_mangle]
extern "C" fn state() {
    let mut tamagotchi = unsafe {TAMAGOTCHI.get_or_insert(Default::default()) };
    tamagotchi.fed = tamagotchi.get_current_feed();
    tamagotchi.entertained = tamagotchi.get_current_entertained();
    tamagotchi.rested = tamagotchi.get_current_rested();
    msg::reply(tamagotchi, 0).expect("Failed to share state");
}

#[no_mangle]
extern "C" fn handle() {


    let action: TmgAction = msg::load().expect("Unable to decode `TmgAction`");
    let mut tamagotchi = unsafe { TAMAGOTCHI.as_mut().expect("Program hasn't been initialized") };
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
            assert_eq!(
                msg::source(),
                tamagotchi.owner,
                "Only owner can set new_owner"
            );
            tamagotchi.owner = new_owner;
            msg::reply(TmgEvent::Transfer(new_owner), 0)
                .expect("Error in reply `TmgEvent::Transfer`");
        }
        TmgAction::Approve(allowed_account) => {
            assert_eq!(
                msg::source(),
                tamagotchi.owner,
                "Only owner can set allowed_account"
            );
            assert_eq!(
                allowed_account,
                tamagotchi.owner,
                "Please set a new allowed_account"
            );
            tamagotchi.allowed_account = Some(allowed_account);
            msg::reply(TmgEvent::Approve(allowed_account), 0)
                .expect("Error in reply `TmgEvent::Approve`");
        }
        TmgAction::RevokeApproval => {
            assert_eq!(
                msg::source(),
                tamagotchi.owner,
                "Only owner can revoke approval"
            );
            tamagotchi.allowed_account = Some(tamagotchi.owner);
            msg::reply(TmgEvent::RevokeApproval, 0)
                .expect("Error in reply `TmgEvent::RevokeApproval`");
        }
    }
}



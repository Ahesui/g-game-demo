#![no_std]
use gstd::{prelude::*};
pub struct PMetadata;

use gmeta::{In,InOut,Metadata};
impl Metadata for PMetadata{
    type Reply = ();
    type Others = ();
    type Signal = ();
    type Init = In<String>;
    type Handle = InOut<TmgAction, TmgEvent>;
    type State = Tamagotchi;
}

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

use codec::{Decode, Encode};
use scale_info::TypeInfo;
#[derive(Default, Encode, Decode, TypeInfo)]
pub struct Tamagotchi {
    pub name: String,
    pub date_of_birth: u64,
}
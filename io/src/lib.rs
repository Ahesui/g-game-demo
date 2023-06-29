#![no_std]
use gstd::{prelude::*,ActorId};
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
   Feed,
   Play,
   Sleep,
   Transfer(ActorId),
   Approve(ActorId),
   RevokeApproval,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum TmgEvent {
   Name(String),
   Age(u64),
   Fed,
   Entertained,
   Slept,
   Transfer(ActorId),
   Approve(ActorId),
   RevokeApproval,
}

use codec::{Decode, Encode};
use scale_info::TypeInfo;
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

#[derive(Debug, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum TamagotchiState {
    TamagotchiFeedState,
    TamagotchiSleepState,
    TamagotchiPlayState
}
#[derive(Debug, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum TamagotchiFeedState {
    EnoughFeed,
    NomalFeed,
    LackFeed
}
#[derive(Debug, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum TamagotchiSleepState {
    EnoughSleep,
    NomalSleep,
    LackSleep
}
#[derive(Debug, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum TamagotchiPlayState {
    EnoughPlay,
    NomalPlay,
    LackPlay
}
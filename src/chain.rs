extern crate time;
extern crate serde;
extern crate serde_json;
extern crate sha2;


use sha2::{Sha256,Digest};
use std::fmt::Write;

#[ derive (Debug ,Clone,Serialize)]

struct Transcation  {
    sender: String,
    reciever: String,
    amount:f32,

}

#[ derive(Serialize,Debug)]
 pub struct Blockchain {
    timestamp:i64,
    nonce:u32,
    pre_hash:String,
    merkle: String,
    difficulty:u32,

 }

 pub struct Block {
    header: Blockchain,
    count:u32,
    transaction: Vec<Transcation>

 }

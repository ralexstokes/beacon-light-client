use std::format;
use std::fs;
mod node_discovery;
mod http_requests;
mod query_node;
mod types;
mod build_objects;
use std::mem;
use std::option;
use crate::types::{BeaconBlockHeader,LightClientStore, LightClientSnapshot,
    SyncCommittee, LightClientUpdate, BeaconState};



fn main(){
    
    // set basic vars and get api key from secret
    let (node_id, node_number) = node_discovery::get_random_node_id(10, 8000);
    let api_key: String = fs::read_to_string(format!("/home/joe/.lighthouse/local-testnet/node_{}/validators/api-token.txt",node_number.to_string())).expect("Nope"); 

    initialize(&api_key, &node_id);

    // TODO:
    // update() currently executes indiscriminately
    // need to make this optional - is an update necessary? If not don't execute!!
    //update(store, &api_key, &node_id);

    
    
}


pub fn initialize(api_key: &str, node_id: &str){
    // initialize() builds a snapshot and store object for the most recent finalized chackpoint
    // the initial store object has no updates in the valid_updates field. This is
    // populated by update()
    let state_id = "finalized".to_string();
    let (current_state_json, current_state) = query_node::get_full_state_object(&api_key, &node_id, &state_id);
    let current_snapshot = build_objects::make_snapshot(&current_state);
    //let store = build_objects::initialize_store(current_snapshot);

    println!("{:?}",&current_snapshot.current_sync_committee.aggregate_pubkey);

    
}


pub fn update(current_store: LightClientStore, api_key: &str, node_id: &str){
    // update takes the initial store object and fills the valid_updates field
    // by querying the head of the chain. We can keep calling update() to 
    // refresh new_store with up to date information.

    let state_id = "head".to_string();
    let (new_state_json, new_state) = query_node::get_full_state_object(&api_key, &node_id, &state_id);
    let beacon_block_body = query_node::get_block_body(&api_key, &node_id, &state_id);
    let new_snapshot = build_objects::make_snapshot(&new_state);
    //let update = build_objects::get_update(&new_state, &new_snapshot, &beacon_block_body);
    
    //let new_store = build_objects::update_store(current_store, new_snapshot, update);

    //return new_store
}

// 5620
// 5631
// 5646
// 5656
// 5666
// 5677
// 5685

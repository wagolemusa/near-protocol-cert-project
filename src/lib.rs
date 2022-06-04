use near_contract_standards::fungible_token::events;
use near_sdk::{
    borsh:: *,
    serde:: *,
    env,
    *, collections::*,
};


// Declearing account
pub type AccountId = String;


use std::{collections::HashMap, default};
// Struct for creating events 
pub use std::fmt::format;
#[derive(BorshDeserialize, BorshSerialize, Debug, Serialize, Deserialize)]
#[serde(crate="near_sdk::serde")]
pub struct Event {
    event_id: u32,
    title: String,
    started_time: String,
    ended_time: String,
    users: Vec<User>,

}


// Struct for creating users who have to join evnets
#[derive(BorshDeserialize, BorshSerialize, Debug, Serialize, Deserialize)]
#[serde(crate="near_sdk::serde")]
pub struct User{
    name: String,
    username: String,
    email: String,   
    user_id: String,
}

// struct to declear events and users
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]

pub struct SmartEvent {
    events: UnorderedMap<String, Event>,
    users: Vec<User>
}


impl Default for SmartEvent {

    fn default() -> Self {
        Self { events: UnorderedMap::new(b"e"), users: Vec::new() }

    }
}
#[near_bindgen]
impl SmartEvent {

    pub fn new_event() -> Self{
        let events: UnorderedMap<String, Event> = UnorderedMap::new(b"e");
        let users: Vec<User> = Vec::new();

        SmartEvent{
            events,
            users
        }
    }

    // Public method to count events 
    pub fn count_events(&self) -> u64 {
        self.events.len()
    }

    // Public method to count users
    pub fn count_users(&self) -> usize {
        self.users.len()
    }

    // Public method to create events and save them in a vector
    pub fn create_event(&mut self, title: String,  
        started_time: String, 
        ended_time: String)
    {
        let mut len = self.events.len();
        let event_id =  len + 1;
        let event1 = Event{
            event_id: event_id as u32, 
            title: title.to_string(),
            started_time: started_time.to_string(),
            ended_time: ended_time.to_string(),
            users: Vec::new(),
        };
        self.events.insert(&event1.title, &event1);

        env::log_str("Event was created succesfully");
        let msg = format!("Your ID is{}", &event_id);
        env::log_str(&msg);

    }

    // Methods to display events
    pub fn show_events(&mut self)-> &Vector<Event> {
        self.events.values_as_vector()
    }

    // Public method to create users and save them in vectors
    pub fn check_in_user(&mut self, name: String, username: String,  email: String){
        let user_id = env::signer_account_id();

        let user1 = User{
            user_id: user_id.to_string(),
            name: name,
            username: username,
            email: email,
            //check if user exits
        };
        self.users.push(user1);
        env::log_str("User Created Succesfully");
    } 

    // Method to display users
    pub fn get_users(&self) -> &Vec<User>{ 
       &self.users 
    
    }

    fn check_in_user_to_event(&mut self, event_title: &String, name: String, username: String,  email: String){


      match self.events.get(event_title) {
    Some(event) => {


            let user_id = env::signer_account_id();
            let user1 = User{
                user_id: user_id.to_string(),
                name: name,
                username: username,
                email: email,
                //check if user exits
        };
        event.users.push(user1);
    },
    None => ()
}

    }

} 


// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::{test_utils::*,  AccountId};
    
    fn get_context(account: AccountId) -> VMContextBuilder{
        let mut bulder = VMContextBuilder::new();
        bulder.signer_account_id(account);
        return bulder;
    }
    
    // Method to test evnets
    #[test]
    fn add_event(){
        let username = AccountId::new_unchecked("djrefuge.testnet".to_string());
        let _context = get_context(username.clone());

        let mut contract = SmartEvent::new_event();
        contract.create_event("BlockChain event".to_string(),  "10:30 pm".to_string(), "12:30 pm".to_string());
        assert_eq!(contract.count_events(), 1);

    }

    // Method to test users
    #[test]
    fn create_user(){
        let username = AccountId::new_unchecked("djrefuge.testnet".to_string());
        let _context = get_context(username.clone());
        let mut contract = SmartEvent::new_event();
        contract.check_in_user("refuge".to_string(), "homie".to_string(), "refuge@gmail.com".to_string());
       let results = contract.count_users();
        assert_eq!(results, 1);
    }   

    #[test]
    fn get_users(){
        let username = AccountId::new_unchecked("djrefuge.testnet".to_string());
        let _context = get_context(username.clone());
        let mut contract = SmartEvent::new_event();
        contract.check_in_user("refuge".to_string(), "homie".to_string(), "refuge@gmail.com".to_string());
         contract.check_in_user("musa".to_string(), "wagole".to_string(), "wagole@gmail.com".to_string());
        
        let count = contract.get_users();
        assert_eq!(count.len(), 2);

    }

    


}
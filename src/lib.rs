use near_sdk::{
    borsh:: *,
    serde:: *,
    env,
    *,
};


// Declearing account
pub type AccountId = String;


// Struct for creating events 
pub use std::fmt::format;
#[derive(BorshDeserialize, BorshSerialize, Debug, Serialize, Deserialize)]
#[serde(crate="near_sdk::serde")]
pub struct Event {
    title: String,
    description: String,
    started_date: String,
    started_time: String,
    ended_time: String,
    user: Vec<User>,

}


// Struct for creating users who have to join evnets
#[derive(BorshDeserialize, BorshSerialize, Debug, Serialize, Deserialize)]
#[serde(crate="near_sdk::serde")]
pub struct User{
    name: String,
    username: String,
    email: String,          
}


// struct to declear events and users
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Default)]
pub struct SmartEvent {
    events: Vec<Event>,
    users: Vec<User>
}


#[near_bindgen]
impl SmartEvent {
    #[init]

    pub fn new_event() -> Self{
        let events: Vec<Event> = Vec::new();
        let users: Vec<User> = Vec::new();

        SmartEvent{
            events,
            users
        }
    }

    // Public method to count events 
    pub fn count_events(&mut self) -> usize {
        self.events.len()
    }

    // Public method to count users
    pub fn count_users(&mut self) -> usize {
        self.users.len()
    }

    // Public method to create events and save them in a vector
    pub fn create_event(&mut self, title: String, description: String, 
        started_date: String, started_time: String, 
        ended_time: String)
    {
        let event1 = Event{
            title: title.to_string(),
            description: description.to_string(),
            started_date: started_date.to_string(),
            started_time: started_time.to_string(),
            ended_time: ended_time.to_string(),
            user: vec![],
        };
        self.events.push(event1);
        env::log_str("Event was created succesfully");
    }

    // Public method to create users and save them in vectors
    pub fn check_in_user(&mut self, name: String, username: String,  email: String){
        let user1 = User{
            name: name.to_string(),
            username: username.to_string(),
            email: email.to_string(),
        };
        self.users.push(user1);
        env::log_str("User Created Succesfully");
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
        contract.create_event("BlockChain event".to_string(), "Come and lean Smart contract".to_string(), "20/6/2022".to_string(), "10:30 pm".to_string(), "12:30 pm".to_string());
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


}
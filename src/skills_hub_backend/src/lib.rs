// Import necessary libraries and modules
use std::collections::HashMap;
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk_macros::{update, query};

// Define a structure to represent a user
#[derive(CandidType, Deserialize, Clone)]
struct User {
    id: String,
    skills: Vec<String>,
    wants_to_learn: Vec<String>,
}

// Define a structure to represent a learning resource
#[derive(CandidType, Deserialize, Clone)]
struct Resource {
    link: String,
    category: String,
    added_by: String,
}

// Main state struct that holds users and resources
#[derive(Default)]
struct SkillExchange {
    users: HashMap<String, User>,
    resources: HashMap<String, Vec<Resource>>,
}

// Declare the state variable using thread-local storage
thread_local! {
    static STATE: SkillExchange = SkillExchange::default();
}

// Function to add a new user
#[update]
fn add_user(id: String, skills: Vec<String>, wants_to_learn: Vec<String>) {
    let user = User { id: id.clone(), skills, wants_to_learn };
    STATE.with(|state| state.users.borrow_mut().insert(id, user));
}

// Function to find matching users based on learning interests
#[query]
fn find_matches(user_id: String) -> Vec<User> {
    STATE.with(|state| {
        let mut matches = Vec::new();
        if let Some(user) = state.users.get(&user_id) {
            for other_user in state.users.values() {
                if other_user.id != user_id
                    && user.wants_to_learn.iter().any(|skill| other_user.skills.contains(skill))
                {
                    matches.push(other_user.clone());
                }
            }
        }
        matches
    })
}

// Function to add a new learning resource
#[update]
fn add_resource(link: String, category: String, added_by: String) {
    let resource = Resource { link, category: category.clone(), added_by };
    STATE.with(|state| {
        state.resources
            .borrow_mut()
            .entry(category)
            .or_insert(Vec::new())
            .push(resource);
    });
}

// Function to retrieve all resources by category
#[query]
fn get_resources(category: String) -> Vec<Resource> {
    STATE.with(|state| state.resources.get(&category).cloned().unwrap_or_default())
}

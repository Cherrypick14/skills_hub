use candid::{CandidType, Deserialize};
use ic_cdk_macros::{query, update};
use std::cell::RefCell;
use std::collections::HashMap;

// Define structures for User and Resource
#[derive(CandidType, Deserialize, Clone)]
struct User {
    id: String,
    skills: Vec<String>,
    wants_to_learn: Vec<String>,
}

#[derive(CandidType, Deserialize, Clone)]
struct Resource {
    link: String,
    category: String,
    added_by: String,
}

// State to hold users and resources
#[derive(Default)]
struct SkillExchange {
    users: HashMap<String, User>,
    resources: HashMap<String, Vec<Resource>>,
}

thread_local! {
    static STATE: RefCell<SkillExchange> = RefCell::new(SkillExchange::default());
}

// Utility function to get user by ID
fn get_user_by_id(user_id: &str) -> Result<User, String> {
    STATE.with(|state| {
        let state = state.borrow();
        state.users.get(user_id).cloned().ok_or_else(|| "User not found.".to_string())
    })
}

// Add a new user
#[update]
fn add_user(id: String, skills: Vec<String>, wants_to_learn: Vec<String>) -> Result<String, String> {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        if state.users.insert(id.clone(), User { id, skills, wants_to_learn }).is_some() {
            Err("User already exists.".to_string())
        } else {
            Ok("User added successfully.".to_string())
        }
    })
}

// Find matching users based on learning interests
#[query]
fn find_matches(user_id: String) -> Result<Vec<User>, String> {
    let user = get_user_by_id(&user_id)?;
    STATE.with(|state| {
        let state = state.borrow();
        let matches = state.users.values()
            .filter(|other| other.id != user.id && user.wants_to_learn.iter().any(|skill| other.skills.contains(skill)))
            .cloned()
            .collect::<Vec<_>>();
        if matches.is_empty() {
            Err("No matches found.".to_string())
        } else {
            Ok(matches)
        }
    })
}

// Add a new learning resource
#[update]
fn add_resource(link: String, category: String, added_by: String) -> Result<String, String> {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.resources.entry(category.clone()).or_default().push(Resource { link, category, added_by });
        Ok("Resource added successfully.".to_string())
    })
}

// Retrieve resources by category
#[query]
fn get_resources(category: String) -> Result<Vec<Resource>, String> {
    STATE.with(|state| {
        state.borrow().resources.get(&category).cloned().ok_or_else(|| "No resources found for this category.".to_string())
    })
}

// Update an existing user's profile
#[update]
fn update_user(id: String, new_skills: Vec<String>, new_wants_to_learn: Vec<String>) -> Result<String, String> {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        if let Some(user) = state.users.get_mut(&id) {
            user.skills = new_skills;
            user.wants_to_learn = new_wants_to_learn;
            Ok("User profile updated successfully.".to_string())
        } else {
            Err("User not found.".to_string())
        }
    })
}



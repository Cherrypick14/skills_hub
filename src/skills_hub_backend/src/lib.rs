use candid::{CandidType, Deserialize};
use ic_cdk_macros::{query, update};
use std::cell::RefCell;
use std::collections::HashMap;

// Define structures for User and Resource
#[derive(CandidType, Deserialize, Clone, Debug)]
struct User {
    id: String,
    skills: Vec<String>,
    wants_to_learn: Vec<String>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
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
        state.users.get(user_id).cloned().ok_or_else(|| format!("User with ID {} not found.", user_id))
    })
}

// Add a new user
#[update]
fn add_user(id: String, skills: Vec<String>, wants_to_learn: Vec<String>) -> Result<String, String> {
    if id.is_empty() || skills.is_empty() || wants_to_learn.is_empty() {
        return Err("Invalid input: All fields must be non-empty.".to_string());
    }

    STATE.with(|state| {
        let mut state = state.borrow_mut();
        if state.users.contains_key(&id) {
            Err(format!("User with ID {} already exists.", id))
        } else {
            state.users.insert(id.clone(), User { id: id.clone(), skills, wants_to_learn });
            Ok(format!("User with ID {} added successfully.", id))
        }
    })
}

// Find matching users based on learning interests
#[query]
fn find_matches(user_id: String) -> Result<Vec<User>, String> {
    let user = get_user_by_id(&user_id)?;
    STATE.with(|state| {
        let state = state.borrow();
        let matches: Vec<User> = state.users.values()
            .filter(|other| {
                other.id != user.id && 
                user.wants_to_learn.iter().any(|skill| other.skills.contains(skill))
            })
            .cloned()
            .collect();

        if matches.is_empty() {
            Err(format!("No matches found for user with ID {}.", user_id))
        } else {
            Ok(matches)
        }
    })
}

// Add a new learning resource
#[update]
fn add_resource(link: String, category: String, added_by: String) -> Result<String, String> {
    if link.is_empty() || category.is_empty() || added_by.is_empty() {
        return Err("Invalid input: All fields must be non-empty.".to_string());
    }

    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.resources.entry(category.clone())
            .or_default()
            .push(Resource { link: link.clone(), category: category.clone(), added_by });
        Ok(format!("Resource '{}' added successfully to category '{}'.", link, category))
    })
}

// Retrieve resources by category
#[query]
fn get_resources(category: String) -> Result<Vec<Resource>, String> {
    STATE.with(|state| {
        state.borrow().resources.get(&category)
            .cloned()
            .ok_or_else(|| format!("No resources found for category '{}'.", category))
    })
}

// Update an existing user's profile
#[update]
fn update_user(id: String, new_skills: Vec<String>, new_wants_to_learn: Vec<String>) -> Result<String, String> {
    if new_skills.is_empty() || new_wants_to_learn.is_empty() {
        return Err("Invalid input: Skills and wants_to_learn must be non-empty.".to_string());
    }

    STATE.with(|state| {
        let mut state = state.borrow_mut();
        if let Some(user) = state.users.get_mut(&id) {
            user.skills = new_skills;
            user.wants_to_learn = new_wants_to_learn;
            Ok(format!("User profile with ID {} updated successfully.", id))
        } else {
            Err(format!("User with ID {} not found.", id))
        }
    })
}
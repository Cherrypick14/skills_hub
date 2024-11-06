use candid::{CandidType, Deserialize};
use ic_cdk_macros::{query, update};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

// Define structures for User and Resource
#[derive(CandidType, Deserialize, Clone, Debug)]
struct User {
    id: String,
    skills: HashSet<String>,
    wants_to_learn: HashSet<String>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
struct Resource {
    id: String,
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

// Utility function to generate unique user IDs
fn generate_unique_id() -> String {
    Uuid::new_v4().to_string()
}

// Utility function to get user by ID
fn get_user_by_id(user_id: &str) -> Result<User, String> {
    STATE.with(|state| {
        let state = state.borrow();
        state.users.get(user_id).cloned().ok_or_else(|| format!("User with ID {} not found.", user_id))
    })
}

// Add a new user with unique ID generation and validation
#[update]
fn add_user(skills: Vec<String>, wants_to_learn: Vec<String>) -> Result<String, String> {
    if skills.is_empty() || wants_to_learn.is_empty() {
        return Err("Skills and wants_to_learn fields must not be empty.".to_string());
    }

    let user_id = generate_unique_id();
    let user = User {
        id: user_id.clone(),
        skills: skills.into_iter().collect(),
        wants_to_learn: wants_to_learn.into_iter().collect(),
    };

    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.users.insert(user_id.clone(), user);
        Ok(format!("User with ID {} added successfully.", user_id))
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
                !user.wants_to_learn.is_disjoint(&other.skills)
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

// Add a new learning resource with unique ID
#[update]
fn add_resource(link: String, category: String, added_by: String) -> Result<String, String> {
    if link.is_empty() || category.is_empty() || added_by.is_empty() {
        return Err("All fields must be non-empty.".to_string());
    }

    let resource_id = generate_unique_id();
    let resource = Resource {
        id: resource_id.clone(),
        link: link.clone(),
        category: category.clone(),
        added_by,
    };

    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.resources.entry(category.clone())
            .or_default()
            .push(resource);
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
        return Err("Skills and wants_to_learn must be non-empty.".to_string());
    }

    STATE.with(|state| {
        let mut state = state.borrow_mut();
        if let Some(user) = state.users.get_mut(&id) {
            user.skills = new_skills.into_iter().collect();
            user.wants_to_learn = new_wants_to_learn.into_iter().collect();
            Ok(format!("User profile with ID {} updated successfully.", id))
        } else {
            Err(format!("User with ID {} not found.", id))
        }
    })
}

// Get all users
#[query]
fn get_all_users() -> Vec<User> {
    STATE.with(|state| {
        state.borrow().users.values().cloned().collect()
    })
}

// Delete a resource by ID
#[update]
fn delete_resource(resource_id: String, category: String) -> Result<String, String> {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        if let Some(resources) = state.resources.get_mut(&category) {
            if let Some(index) = resources.iter().position(|r| r.id == resource_id) {
                resources.remove(index);
                return Ok(format!("Resource with ID {} deleted successfully.", resource_id));
            }
        }
        Err(format!("Resource with ID {} not found in category '{}'.", resource_id, category))
    })
}

// Check if a user exists by ID
#[query]
fn user_exists(user_id: String) -> bool {
    STATE.with(|state| state.borrow().users.contains_key(&user_id))
}

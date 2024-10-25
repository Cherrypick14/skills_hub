// Import necessary libraries and modules
use std::collections::HashMap;
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk_macros::{update, query};

// Define a structure to represent a user in the skill exchange system
#[derive(CandidType, Deserialize, Clone)]
struct User {
    id: String,                     
    skills: Vec<String>,            
    wants_to_learn: Vec<String>,    
}

// Define a structure to represent a review left by one user for another
#[derive(CandidType, Deserialize, Clone)]
struct Review {
    reviewer: String,               
    rating: u8,                    
    comment: String,               
}

// Define a structure to represent a learning resource shared in the platform
#[derive(CandidType, Deserialize, Clone)]
struct Resource {
    link: String,                   
    category: String,               
    description: String,            
    added_by: String,              
}

// Main state struct that holds users, reviews, and resources in the platform
#[derive(Default)]
struct SkillExchange {
    users: HashMap<String, User>,        
    reviews: HashMap<String, Vec<Review>>, 
    resources: HashMap<String, Vec<Resource>>, 
}

// Declare the state variable using thread-local storage
thread_local! {
    static STATE: SkillExchange = SkillExchange::default();
}

// Function to add a new user to the platform
#[update]
fn add_user(id: String, skills: Vec<String>, wants_to_learn: Vec<String>) {
    let user = User { id: id.clone(), skills, wants_to_learn };
    STATE.with_mut(|state| state.users.insert(id, user)); 
}

// Function to find matching users based on shared learning interests
#[query]
fn find_matches(user_id: String) -> Vec<User> {
    let mut matches = Vec::new();
    STATE.with(|state| {
        // Ensure the user exists before proceeding
        if let Some(user) = state.users.get(&user_id) {
            // Iterate through all users and find those who have skills the user wants to learn
            for other_user in state.users.values() {
                if other_user.id != user_id && 
                   user.wants_to_learn.iter().any(|skill| other_user.skills.contains(skill)) {
                    matches.push(other_user.clone()); 
                }
            }
        }
    });
    matches
}

// Function to submit a review for another user
#[update]
fn submit_review(reviewee: String, reviewer: String, rating: u8, comment: String) {
    let review = Review { reviewer, rating, comment };
    STATE.with_mut(|state| {
        // Add the review to the list of reviews for the specified user
        state.reviews.entry(reviewee.clone()).or_insert(Vec::new()).push(review);
    });
}

// Function to retrieve all reviews for a specific user
#[query]
fn get_reviews(user_id: String) -> Vec<Review> {
    STATE.with(|state| state.reviews.get(&user_id).cloned().unwrap_or_default())
}

// Function to add a new learning resource to the platform
#[update]
fn add_resource(link: String, category: String, description: String, added_by: String) {
    let resource = Resource { link, category: category.clone(), description, added_by };
    STATE.with_mut(|state| {
        // Add the resource to the specified category
        state.resources.entry(category).or_insert(Vec::new()).push(resource);
    });
}

// Function to retrieve all resources for a specific category
#[query]
fn get_resources(category: String) -> Vec<Resource> {
    STATE.with(|state| state.resources.get(&category).cloned().unwrap_or_default())
}

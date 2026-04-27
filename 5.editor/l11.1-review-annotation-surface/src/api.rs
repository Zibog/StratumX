use crate::review::*;
use std::collections::HashMap;

pub struct ReviewService {
    reviews: HashMap<uuid::Uuid, Review>,
}

impl ReviewService {
    pub fn new() -> Self {
        Self {
            reviews: HashMap::new(),
        }
    }

    pub fn create_review(&mut self, title: String) -> uuid::Uuid {
        let review = Review::new(title);
        let id = review.id;
        self.reviews.insert(id, review);
        id
    }
}

impl Default for ReviewService {
    fn default() -> Self {
        Self::new()
    }
}
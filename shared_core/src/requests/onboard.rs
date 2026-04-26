use serde::{Deserialize, Serialize};

/// Represents the data required to create a new organization and its first user.
#[derive(Debug, Serialize, Deserialize)]
pub struct OnboardingRequest {
    pub organization_name: String,
    pub user_email: String,
    pub user_password: String,
    pub user_full_name: String,
    pub user_display_name: Option<String>,
    pub coa_template_id: String,
}

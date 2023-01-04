// Struck that represents a common issue interface between Drivers
pub struct Issue {
    pub id: String,
    pub title: String,
    // Add this extra fields ASA we add support for real drivers
    // pub description: Option<String>,
    // pub url: Option<String>,
    // pub status: Option<String>,
    // pub assignee: Option<String>,
    // pub reporter: Option<String>,
    // pub created: Option<String>,
    // pub updated: Option<String>,
}

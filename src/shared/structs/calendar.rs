use super::location;

use serde::{
    Serialize,
    Deserialize,
};
use chrono::{
    DateTime,
    offset,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CalendarItem {
    pub title: String,
    pub start: DateTime<offset::Utc>,
    pub end: DateTime<offset::Utc>,
    // Guests will be a link to INS or contact or something
    pub guests: (),
    pub location: location::Location,
    pub description: String,
    // Attachments will link to either a sharing link or local file.
    // Sharing links will be visible to all with permissions to read
    // it. Maybe add something that automatically offers to add permissions to guests.
    // Local file will only be available to owner. Maybe add automatic generation
    // of a sharing link eg. local_file->upload_to_cloud->generate_link
    pub attachments: (),
    // Action will link to some action from another interface eg. Call, meeting link etc..
    pub action: (),
    // Guests will be a link to INS or contact or something
    pub owner: ()
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CalendarAction {
    Single, 
    Array,
    Range,
}
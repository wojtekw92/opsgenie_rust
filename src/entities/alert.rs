extern crate reqwest;
use reqwest::Client;
use reqwest::header::AUTHORIZATION;
use serde::{Serialize, Deserialize};


#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
struct AlertCreateResponse {
    result: String,
    took: f32,
    request_id: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Priority {
    P1,
    P2,
    P3,
    P4,
    P5,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Responder {
    id: String,
    r#type: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct AlertData {
    /// Message of the alert
    pub message: String,
    /// Client-defined identifier of the alert, that is also the key element of Alert De-Duplication.
    #[serde(skip_serializing_if="Option::is_none")]
    pub alias: Option<String>,
    /// Description field of the alert that is generally used to provide a detailed information about the alert.
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    /// Teams, users, escalations and schedules that the alert will be routed to send notifications.
    /// type field is mandatory for each item, where possible values are team, user, escalation and schedule.
    /// If the API Key belongs to a team integration, this field will be overwritten with the owner team.
    /// Either id or name of each responder should be provided.You can refer below for example values.
    #[serde(skip_serializing_if="Option::is_none")]
    pub responders: Option<Vec<Responder>>,
    /// Teams and users that the alert will become visible to without sending any notification.type
    /// field is mandatory for each item, where possible values are team and user.
    /// In addition to the type field, either id or name should be given for teams and either
    /// id or username should be given for users. Please note: that alert will be visible to
    /// the teams that are specified withinresponders field by default, so there is no need
    /// to re-specify them within visibleTo field. You can refer below for example values.
    #[serde(skip_serializing_if="Option::is_none")]
    pub visible_to: Option<Vec<Responder>>,
    /// Custom actions that will be available for the alert.
    #[serde(skip_serializing_if="Option::is_none")]
    pub actions: Option<Vec<String>>,
    /// Tags of the alert.
    #[serde(skip_serializing_if="Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Map of key-value pairs to use as custom properties of the alert.
    #[serde(skip_serializing_if="Option::is_none")]
    pub details: Option <String>, // TODO: that should be key value pair.
    /// Entity field of the alert that is generally used to specify which domain alert is related to.
    #[serde(skip_serializing_if="Option::is_none")]
    pub entity: Option <String>,
    /// Source field of the alert. Default value is IP address of the incoming request.
    #[serde(skip_serializing_if="Option::is_none")]
    pub source: Option <String>,
    /// Priority level of the alert. Possible values are P1, P2, P3, P4 and P5. Default value is P3.
    #[serde(skip_serializing_if="Option::is_none")]
    pub priority: Option <Priority>,
    /// Display name of the request owner.
    #[serde(skip_serializing_if="Option::is_none")]
    pub user: Option <String>,
    /// Additional note that will be added while creating the alert.
    #[serde(skip_serializing_if="Option::is_none")]
    pub note: Option <String>,
}

pub struct Alert {
    request_id: String,
}
impl Alert {
    pub fn create(key: String, data: AlertData) -> Result<Alert, reqwest::Error> {
        let request_url="https://api.opsgenie.com/v2/alerts";
        let mut response = Client::new()
            .post(request_url)
            .header(AUTHORIZATION, format!("GenieKey {}", key))
            .json(&data)
            .send()?;
         let resp: AlertCreateResponse = response.json()?;
         println!("{:?}", resp);
        Ok(Alert {
            request_id: resp.request_id,
        })
    }
}
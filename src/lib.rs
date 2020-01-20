mod entities;

pub use entities::alert::AlertData as AlertData;
pub use entities::alert::Alert as Alert;
pub use entities::alert::Priority as Priority;



/// OpsGenie API object
pub struct OpsGenie {
    /// ops genie API key
    api_key: String,
}

impl OpsGenie {
    /// Return new OpsGenie object
    ///
    /// # Arguments
    ///
    /// * `key` - ops genie API key
    ///
    /// # Example
    ///
    /// ```
    /// use opsgenie_rust::OpsGenie;
    /// let ops_genie = OpsGenie::new("<API_KEY>");
    /// ```
    pub fn new(key: String) -> OpsGenie {
        OpsGenie {
            api_key: key.clone(),
        }
    }
    pub fn alert(&self, alert_data: AlertData) -> Result<Alert, reqwest::Error> {
        Alert::create(&self.api_key, alert_data)
    }
}
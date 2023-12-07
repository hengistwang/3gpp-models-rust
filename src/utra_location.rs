use serde::{Deserialize, Serialize};

/// UtraLocation : Exactly one of cgi, sai or lai shall be present.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UtraLocation {
    #[serde(rename = "cgi", skip_serializing_if = "Option::is_none")]
    pub cgi: Option<Box<crate::CellGlobalId>>,
    #[serde(rename = "sai", skip_serializing_if = "Option::is_none")]
    pub sai: Option<Box<crate::ServiceAreaId>>,
    #[serde(rename = "lai", skip_serializing_if = "Option::is_none")]
    pub lai: Option<Box<crate::LocationAreaId>>,
    #[serde(rename = "rai", skip_serializing_if = "Option::is_none")]
    pub rai: Option<Box<crate::RoutingAreaId>>,
    /// The value represents the elapsed time in minutes since the last network contact of the mobile station.  Value \"0\" indicates that the location information was obtained after a successful paging procedure for  Active Location Retrieval when the UE is in idle mode  or after a successful location reporting procedure  the UE is in connected mode. Any other value than \"0\" indicates that the location information is the last known one.  See 3GPP TS 29.002 clause 17.7.8.
    #[serde(
        rename = "ageOfLocationInformation",
        skip_serializing_if = "Option::is_none"
    )]
    pub age_of_location_information: Option<i32>,
    /// string with format 'date-time' as defined in OpenAPI.
    #[serde(
        rename = "ueLocationTimestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub ue_location_timestamp: Option<String>,
    /// Refer to geographical Information.See 3GPP TS 23.032 clause 7.3.2. Only the description of an ellipsoid point with uncertainty circle is allowed to be used.
    #[serde(
        rename = "geographicalInformation",
        skip_serializing_if = "Option::is_none"
    )]
    pub geographical_information: Option<String>,
    /// Refers to Calling Geodetic Location. See ITU-T Recommendation Q.763 (1999) clause 3.88.2. Only the description of an ellipsoid point with uncertainty circle is allowed to be used.
    #[serde(
        rename = "geodeticInformation",
        skip_serializing_if = "Option::is_none"
    )]
    pub geodetic_information: Option<String>,
}

impl UtraLocation {
    /// Exactly one of cgi, sai or lai shall be present.
    pub fn new() -> UtraLocation {
        UtraLocation {
            cgi: None,
            sai: None,
            lai: None,
            rai: None,
            age_of_location_information: None,
            ue_location_timestamp: None,
            geographical_information: None,
            geodetic_information: None,
        }
    }
}
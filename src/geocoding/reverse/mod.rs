//! Resources (enums, structs, methods) for the client to build a Reverse
//! Geocoding API request for the Google Cloud server. Reverse geocoding
//! converts a latitude/longitude to an address.

mod build;
mod execute;
mod get;
mod new;
mod with_language;
mod with_location_types;
mod with_result_types;

use crate::{
    geocoding::location_type::LocationType,
    client_settings::ClientSettings,
    language::Language,
    latlng::LatLng,
    place_type::PlaceType,
}; // use

/// Use this structure's methods to build your Reverse Geocoding API request.
/// Address lookup from latitude/longitude.

#[derive(Debug, PartialEq, PartialOrd)]
pub struct ReverseRequest<'a> {

    // Required parameters:
    // --------------------

    /// This structure contains the application's API key and other
    /// user-definable settings such as "maximum retries."
    client_settings: &'a mut ClientSettings,

    /// The latitude and longitude values specifying the location for which you
    /// wish to obtain the closest, human-readable address.
    latlng: LatLng,

    // Optional parameters:
    // --------------------

    /// The language in which to return results.
    language: Option<Language>,

    /// A filter of one or more location types. If the parameter contains
    /// multiple location types, the API returns all addresses that match any of
    /// the types. A note about processing: The `location_type` parameter does
    /// not restrict the search to the specified location type(s). Rather, the
    /// `location_type` acts as a post-search filter: the API fetches all
    /// results for the specified `latlng`, then discards those results that do
    /// not match the specified location type(s).
    location_types: Option<Vec<LocationType>>,

    /// A filter of one or more address types. If the parameter contains
    /// multiple address types, the API returns all addresses that match any of
    /// the types. A note about processing: The `result_type` parameter does not
    /// restrict the search to the specified address type(s). Rather, the
    /// `result_type` acts as a post-search filter: the API fetches all results
    /// or the specified `latlng`, then discards those results that do not match
    /// the specified address type(s).
    result_types: Option<Vec<PlaceType>>,

    // Internal use only:
    // ------------------

    /// Query string that is to be submitted to the Google Cloud Maps Platform.
    query: Option<String>,

    /// Has the request been validated?
    validated: bool,

} // impl
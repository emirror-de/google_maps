use crate::latlng::LatLng;
use serde::{Serialize, Deserialize};

/// Contains information about the stop/station for this part of the trip.

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransitStop {

    /// The name of the transit station/stop. eg. "Union Square".
    name: String,

    /// The location of the transit station/stop, represented as a `lat` and
    /// `lng` field.
    location: LatLng,

} // struct
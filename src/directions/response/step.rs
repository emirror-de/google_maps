use crate::{
    directions::{
        response::{
            distance::Distance,
            duration::Duration,
            maneuver_type::ManeuverType,
            polyline::Polyline,
            transit_details::TransitDetails,
        }, // response
        travel_mode::TravelMode
    }, // directions
    latlng::LatLng,
}; // use
use serde::{Serialize, Deserialize};

/// Each element in the `steps` array defines a single step of the calculated
/// directions. A step is the most atomic unit of a direction's route,
/// containing a single step describing a specific, single instruction on the
/// journey. E.g. "Turn left at W. 4th St." The step not only describes the
/// instruction but also contains distance and duration information relating to
/// how this step relates to the following step. For example, a step denoted as
/// "Merge onto I-80 West" may contain a duration of "37 miles" and
/// "40 minutes," indicating that the next step is 37 miles/40 minutes from this
/// step.
///
/// When using the Directions API to search for transit directions, the steps
/// array will include additional [transit details](https://developers.google.com/maps/documentation/directions/intro#TransitDetails)
/// in the form of a `transit_details` array. If the directions include multiple
/// modes of transportation, detailed directions will be provided for walking or
/// driving steps in an inner `steps` array. For example, a walking step will
/// include directions from the start and end locations: "Walk to Innes Ave &
/// Fitch St". That step will include detailed walking directions for that route
/// in the inner steps array, such as: "Head north-west", "Turn left onto
/// Arelious Walker", and "Turn left onto Innes Ave".

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Step {

    /// The distance covered by this step. This property may be undefined as the
    /// distance may be unknown.
    distance: Distance,

    /// The typical time required to perform this step in seconds and in text
    /// form. This property may be undefined as the duration may be unknown.
    duration: Duration,

    /// The ending location of this step.
    end_location: LatLng,

    /// Instructions for this step.
    html_instructions: String,

    /// Contains the action to take for the current step (turn left, merge,
    /// straight, etc.). This field is used to determine which icon to display.
    maneuver: Option<ManeuverType>,

    /// Contains a single `points` object that holds an [encoded polyline](https://developers.google.com/maps/documentation/utilities/polylinealgorithm) representation of the step. This polyline is an approximate (smoothed) path of the step. (Corresponds to `path` in the [Directions.Step interface](https://developers.google.com/maps/documentation/javascript/reference/directions#DirectionsStep).)
    polyline: Polyline,

    /// The starting location of this step.
    start_location: LatLng,

    /// Transit-specific details about this step. This property will be
    /// undefined unless the travel mode of this step is TravelMode::Transit.
    transit_details: Option<TransitDetails>,

    /// The mode of travel used in this step.
    travel_mode: TravelMode,

} // struct
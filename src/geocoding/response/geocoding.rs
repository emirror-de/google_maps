use crate::{
    geocoding::response::{
        address_component::AddressComponent,
        geometry::Geometry,
        plus_code::PlusCode,
    }, // geocoding::response
    place_type::PlaceType,
}; // use
use serde::{Serialize, Deserialize};

/// When the geocoder returns results, it places them within a results array.
/// Even if the geocoder returns no results (such as if the address doesn't
/// exist) it still returns an empty results array.

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Geocoding {

    /// Array containing the separate components applicable to this address.
    pub address_components: Vec<AddressComponent>,

    /// A string containing the human-readable address of this location.
    ///
    /// Often this address is equivalent to the postal address. Note that some
    /// countries, such as the United Kingdom, do not allow distribution of true
    /// postal addresses due to licensing restrictions.
    ///
    /// The formatted address is logically composed of one or more address
    /// components. For example, the address "111 8th Avenue, New York, NY"
    /// consists of the following components: "111" (the street number), "8th
    /// Avenue" (the route), "New York" (the city) and "NY" (the US state).
    ///
    /// Do not parse the formatted address programmatically. Instead you should
    /// use the individual address components, which the API response includes
    /// in addition to the formatted address field.
    pub formatted_address: String,

    /// Contains the geocoded latitude/longitude, recommended viewport for
    /// displaying the returned result, the bounding box, and other additional
    /// data. See the `Geometry` struct declaration.
    pub geometry: Geometry,

    /// Indicates that the geocoder did not return an exact match for the
    /// original request, though it was able to match part of the requested
    /// address. You may wish to examine the original request for misspellings
    /// and/or an incomplete address.
    ///
    /// Partial matches most often occur for street addresses that do not exist
    /// within the locality you pass in the request. Partial matches may also be
    /// returned when a request matches two or more locations in the same
    /// locality. For example, "21 Henr St, Bristol, UK" will return a partial
    /// match for both Henry Street and Henrietta Street. Note that if a request
    /// includes a misspelled address component, the geocoding service may
    /// suggest an alternative address. Suggestions triggered in this way will
    /// also be marked as a partial match.
    pub partial_match: Option<bool>,

    /// A unique identifier that can be used with other Google APIs. For
    /// example, you can use the place_id in a [Places
    /// API](https://developers.google.com/places/web-service/details) request
    /// to get details of a local business, such as phone number, opening hours,
    /// user reviews, and more. See the [place ID
    /// overview](https://developers.google.com/places/place-id).
    pub place_id: String,

    /// (See [Open Location
    /// Code](https://en.wikipedia.org/wiki/Open_Location_Code) and [plus
    /// codes](https://plus.codes/)) is an encoded location reference. Plus
    /// codes can be used as a replacement for street addresses in places where
    /// they do not exist (where buildings are not numbered or streets are not
    /// named.)
    pub plus_code: Option<PlusCode>,

    /// Array indicates the `type` of the returned result. This array contains a
    /// set of zero or more tags identifying the type of feature returned in the
    /// result. For example, a geocode of "Chicago" returns "locality" which
    /// indicates that "Chicago" is a city, and also returns "political" which
    /// indicates it is a political entity.
    pub types: Vec<PlaceType>,

} // struct
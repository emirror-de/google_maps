use crate::time_zone::{
    error::Error,
    request::Request,
    // Response is aliased to avoid collision with Isahc namespace.
    response::Response as TimeZoneResponse,
}; // use

impl Request {

    /// Performs the HTTP get request and returns the response to the caller.
    ///
    /// # Arguments:
    ///
    /// This method accepts no arguments.

    pub fn get(&self) -> Result<TimeZoneResponse, Error> {

        // Build the URI stem for the HTTP get request:

        const SERVICE_URI: &str = "https://maps.googleapis.com/maps/api/timezone";
        const OUTPUT_FORMAT: &str = "json"; // json or xml
        let mut uri = format!("{}/{}?", SERVICE_URI, OUTPUT_FORMAT);

        match &self.query {
            // If query string built, append it to the URI stem.
            Some(query) => uri.push_str(query.as_ref()),
            // If query string not built, return an error.
            None => return Err(Error::QueryNotBuilt),
        } // match

        // Query the Google Cloud Maps Platform using using an HTTP get request,
        // and return result to caller:

        // let response = isahc::get(uri)?.json::<TimeZoneResponse>()?;
        let response = reqwest::blocking::get(&*uri)?.json::<TimeZoneResponse>()?;
        Ok(response)

    } // fn

} // impl
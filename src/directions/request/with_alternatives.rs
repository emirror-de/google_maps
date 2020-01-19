use crate::directions::request::Request;

impl Request {

    /// Specify whether service may provide more than one route alternative in
    /// the response.
    ///
    /// Description
    /// -----------
    ///
    /// If set to `true`, specifies that the Directions service may provide more
    /// than one route alternative in the response. Note that providing route
    /// alternatives may increase the response time from the server. This is
    /// only available for requests without intermediate waypoints.
    ///
    /// Example:
    /// ---------
    /// * With alternatives:
    /// ```
    /// .with_alternatives(true)
    /// ```

    pub fn with_alternatives(&mut self, alternatives: bool) -> &mut Request {
        self.alternatives = Some(alternatives);
        self
    } // fn

} // impl
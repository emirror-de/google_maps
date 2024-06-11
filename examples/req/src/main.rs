use google_maps::prelude::*;

#[tokio::main]
async fn main() {
    let google_maps_client = GoogleMapsClient::new("APIKEY");

    // Example request:

    let directions = google_maps_client.directions(
        // Origin: Canadian Museum of Nature
        Location::from_address("240 McLeod St, Ottawa, ON K2P 2R1"),
        // Destination: Canada Science and Technology Museum
        Location::try_from_f32(45.403_509, -75.618_904).unwrap(),
    )
    .with_travel_mode(TravelMode::Driving)
    .execute()
    .await
    .expect("something went wrong");

    // Dump entire response:

    println!("{:#?}", directions);
}

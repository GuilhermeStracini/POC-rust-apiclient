use poc_api_client::client::RestApiClient;
use poc_api_client::models::Post;

fn main(){
    let base_url = "https://jsonplaceholder.typicode.com";
    let client = RestApiClient::new(base_url);

    // GET Request
    let post = client.get_post(1).await.unwrap();
    println!("Fetched Post: {:?}", post);
}

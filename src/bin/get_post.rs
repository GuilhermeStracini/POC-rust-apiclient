use poc_api_client::client::PocApiClient;

#[tokio::main]
async fn main(){
    let base_url = "https://jsonplaceholder.typicode.com";
    let client = PocApiClient::new(base_url);

    // GET Request
    let post = client.get_post(1).await.unwrap(); 
    println!("{:?}", post);
}
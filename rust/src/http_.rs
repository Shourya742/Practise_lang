#[cfg(test)]
mod test {

    use reqwest::blocking::{Client, ClientBuilder};
    use reqwest::redirect::Policy;

    #[test]
    fn test_http() {
        let http_client = Client::new();
        let http_result = http_client.get("https://fakestoreapi.com/products").send();

        if http_result.is_ok() {
            println!("{:#?}", http_result.ok().unwrap().text().unwrap());
        } else {
            println!("{:#?}", http_result.err().unwrap());
        }

        let post_result = http_client
            .post("https://jsonplaceholder.typicode.com/posts")
            .body("{'first_name':'RUST','last_name':'reqwest'}")
            .send();
        println!("{:#?}", post_result.ok().unwrap());

        let redir_policy = Policy::limited(5);

        let _http_client = ClientBuilder::new()
            .redirect(redir_policy)
            .build()
            .ok()
            .unwrap();
    }
}

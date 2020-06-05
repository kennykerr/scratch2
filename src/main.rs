use bindings::windows::foundation::Uri;
use bindings::windows::web::syndication::SyndicationClient;

fn main() -> winrt::Result<()> {
    let uri = Uri::create_uri("https://kennykerr.ca/feed")?;
    let client = SyndicationClient::new()?;
    let feed = client.retrieve_feed_async(uri)?.get()?;

    for item in feed.items()?.into_iter().take(3) {
        println!("title: {}", item.title()?.text()?);
    }
        
    Ok(())
}

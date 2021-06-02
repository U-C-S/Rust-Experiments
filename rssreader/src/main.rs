use bindings::Windows::{Foundation::Uri, Web::Syndication::SyndicationClient};
use std::io;

fn main() -> windows::Result<()> {
    let uri = Uri::CreateUri("https://blogs.windows.com/feed")?;
    let client = SyndicationClient::new()?;
    let feed = client.RetrieveFeedAsync(uri)?.get()?;
    for item in feed.Items()? {
        println!("{}", item.Title()?.Text()?);
    }

    let mut lol = String::from("ok");
    io::stdin().read_line(&mut lol).expect("Ok");

    Ok(())
}

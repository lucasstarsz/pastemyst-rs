// get_paste_async
use pastemyst_rs::paste;
use pastemyst_rs::paste::PasteObject;

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

#[tokio::main]
async fn main() -> Result<()> {
    let paste: PasteObject = paste::get_paste_async("gozvbhzs").await?;
    println!("{}", paste.ownerId);
    Ok(())
}

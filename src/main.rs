use scraper::Html;
use mlua::prelude::*;

fn main() {

    let client = reqwest::blocking::Client::new();

    let response = client.get("http://pastebin.com").send().unwrap();

    let response = response.text().unwrap_or("An error occured.".to_string());

    let doc = Html::parse_document(&response);

    for element in doc.root_element().children() {
        match element.value().as_element() {
            None => {},
            Some(elem) => {
                println!("{}", elem.name());
            }
        }
    }

    let lua = Lua::new();

    lua.load("
        print(\"Hello from lua\")
    ").exec().unwrap();

}

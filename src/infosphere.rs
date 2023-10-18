use scraper::{Html, Selector};

use curl::easy::Easy;

const BASE_URL: &str = "https://theinfosphere.org/api.php";

#[allow(dead_code)]
pub fn get_episodes(from_season: Option<i32>) -> Result<(),Box<dyn std::error::Error>> {
    let data = get_episode_list_from_infosphere(); 

    let document = convert_html_document(data);

    let all_seasons_table_selector = r#"body > div > table.overview"#;
    let selector = Selector::parse(all_seasons_table_selector).unwrap();
    let season = Selector::parse(r#"body > div > h2 > .mw-headline"#).unwrap();
    let mut season_iterator = document.select(&season);
    for table in document.select(&selector).skip(1){
        let episode_selector = Selector::parse(r#"tbody > tr > td > b > a"#).unwrap();
        let mut episode_iterator = table.select(&episode_selector).peekable();
        if episode_iterator.peek().is_some() {
            let season = season_iterator.next().unwrap();
            if let Some(search_season) = from_season {
                if season.inner_html() != format!("Season {}", search_season) {
                    continue
                }
            }
            println!("\n#### {} ####", season.inner_html());
        }

        for element in episode_iterator{
            println!("{}", element.inner_html())
        }
    }
    Ok(())
}

fn get_episode_list_from_infosphere() -> Vec<u8> {
    let mut data = Vec::new();
    let mut easy = Easy::new();
    let url = format!("{}?action=parse&prop=text&page=Episode_Listing&format=json", BASE_URL);
    easy.url(&url).unwrap();
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }

    data
}

fn convert_html_document(data: Vec<u8>) -> Html {
    let json = String::from_utf8(data).unwrap();
    let parsed = json::parse(&json).unwrap();
    
    let text = &parsed["parse"]["text"]["*"].as_str().unwrap();
    let document = Html::parse_document(text);

    document
}

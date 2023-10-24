use scraper::{Html, Selector, ElementRef};
use urlencoding::encode;
use curl::easy::Easy;

const BASE_URL: &str = "https://theinfosphere.org";

pub fn get_episodes(from_season: Option<i32>) -> Result<(),Box<dyn std::error::Error>> {
    let document = get_episode_list_html_from_infosphere();

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

pub fn describe_episode(episode_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let episode_path = get_encoded_title(episode_name);

    let document = get_episode_html_from_infosphere(&episode_path);

    let title_selector = Selector::parse(r#"div > table.infobox.alternateable > tbody > tr:nth-child(2) > th"#).unwrap();
    let title = document.select(&title_selector).next().expect("Could not find episode, please check your episode title and try again");
    println!("{}", title.inner_html());

    println!("\nINFO\n----");
    
    // dumps the page html for testing
    // println!("{}", document.html());
    //
    let info_selector = Selector::parse(r#"div > p"#).unwrap();
    let info = document.select(&info_selector).next().unwrap();

    //todo get just the text from the element
	let info = clean_html(&info.inner_html());
    println!("{}", info);

    println!("\nPLOT\n----");
    let selector = Selector::parse(r#"div > h3 > .mw-headline"#).unwrap();
    for headline in document.select(&selector) {
        if headline.inner_html().contains("Act") {
            println!("{}\n", headline.inner_html());

            let parent = headline.parent().unwrap();
            for sibling in parent.next_siblings() {
                let element = match ElementRef::wrap(sibling){
                    None => {continue;},
                    Some(element) => {
                        element
                    }

                };
                if element.value().name() == "p" {
                    let p = element.inner_html(); 
                    let p = clean_html(&p);
                    println!("{}\n\n", p);
                } else if element.value().name() == "div" {
                    continue;
                } 
                else {
                    break;
                }
            }
        }
        else {
            break;
        }
    }

    println!("\nLINK\n----");
    println!("{}/{}", BASE_URL, episode_path);
    Ok(())
}

fn get_episode_list_html_from_infosphere() -> Html {
    let url = format!("{}/api.php?action=parse&prop=text&page=Episode_Listing&format=json", BASE_URL);
    let data = get_from_infosphere(&url);
    convert_html_document(data)
}

fn get_episode_html_from_infosphere(page: &str) -> Html {
    let url = format!("{}/api.php?action=parse&prop=text&page={}&format=json", BASE_URL, page);
    let data = get_from_infosphere(&url);
    convert_html_fragment(data)
}

fn get_encoded_title(title: &str) -> String {
    let title = title.replace(" ", "_");
    let encoded = encode(&title);

    return encoded.into_owned()

}

fn get_from_infosphere(path: &str) -> Vec<u8> {
    let mut data = Vec::new();
    let mut easy = Easy::new();
    easy.url(path).unwrap();
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

fn convert_html_fragment(data: Vec<u8>) -> Html {
    let json = String::from_utf8(data).unwrap();
    let parsed = json::parse(&json).unwrap();
    
    let text = &parsed["parse"]["text"]["*"].as_str().unwrap();
    let document = Html::parse_fragment(text);

    document
}

fn clean_html(data: &str) -> String {
	let data = html_escape::decode_html_entities(&data);
	let data = strip_html(&data);
	let data = trim_whitespace(&data);
    data
}

fn strip_html(source: &str) -> String {
    let mut data = String::new();
    let mut inside = false;
    for c in source.chars() {
        if c == '<' {
            inside = true;
            continue;
        }
        if c == '>' {
            inside = false;
            continue;
        }
        if !inside {
            data.push(c);
        }
    }
    data
}

fn trim_whitespace(s: &str) -> String {
    let mut new_str = s.trim().to_owned();
    let mut prev = ' '; // The initial value doesn't really matter
    new_str.retain(|ch| {
        let result = ch != ' ' || prev != ' ';
        prev = ch;
        result
    });
    new_str
}

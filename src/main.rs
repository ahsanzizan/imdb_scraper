use reqwest::blocking::get;
use scraper::{Html, Selector}; 
use std::fs::File; 
use std::io::Write;

const URL: &str = "https://www.imdb.com/chart/top/";

const FILE_NAME: &str = "movies.csv";

fn main() {
    let response = get(URL)
        .expect("Failed to get the web page")
        .text()
        .expect("Failed to get the response body");

    let document = Html::parse_document(&response);

    let title_selector =
        Selector::parse(".titleColumn > a").expect("Failed to create the title selector");

    let rating_selector =
        Selector::parse(".ratingColumn > strong").expect("Failed to create the rating selector");

    let mut movies: Vec<(String, String)> = Vec::new();

    for (title, rating) in document
        .select(&title_selector)
        .zip(document.select(&rating_selector))
    {
        let title_text = title.text().collect::<Vec<_>>().join("");

        let rating_text = rating.text().collect::<Vec<_>>().join("");

        movies.push((title_text, rating_text));
    }

    let mut file = File::create(FILE_NAME).expect("Failed to create the file");

    file.write_all(b"Title,Rating\n")
        .expect("Failed to write the header");

    for (title, rating) in movies {
        let line = format!("{},{}\n", title, rating);

        // Write the line to the file
        file.write_all(line.as_bytes())
            .expect("Failed to write the line");
    }

    println!(
        "Web scraping done! Check the {} file for the results.",
        FILE_NAME
    );
}

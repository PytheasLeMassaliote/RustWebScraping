use reqwest::Client;
use scraper::{Html, Selector};
use std::collections::HashSet;
use tokio;
use csv::Writer;
use ego_tree::NodeRef;

#[tokio::main]
async fn main() {
    test().await;
}

struct Work {
    author: Option<String>,
    title: Option<String>,
    english: Option<String>,
    /*original: Option<String>,
    domains: Option<String>,
    r#type: Option<String>,
    translator: Option<String>,
    translated_from: Option<String>,
    date: Option<String>,
    publication_type: Option<String>,
    author_editor: Option<String>,
    title_b: Option<String>,
    r#in: Option<String>,
    published: Option<String>,
    volumes: Option<String>,
    pages: Option<String>,*/
}

async fn test() {
    let mut work: Vec<Work> = Vec::new();
    let mut unique_addresses = HashSet::new();
    let client = Client::new();
    let link = client.get("https://www.graeco-arabic-studies.org/texts.html").send().await.unwrap();

    let body = link.text().await.unwrap();
    let document = Html::parse_document(&body);
    let li_selector = Selector::parse("li.l3.l3-c2").unwrap();

    for li in document.select(&li_selector) {
        let a_li = Selector::parse("a").unwrap();
        for a in li.select(&a_li) {
            let href = a.value().attr("href").unwrap_or("");
            if href.starts_with("single-text/") {
                if unique_addresses.insert(href) {
                    let full_address = format!("https://www.graeco-arabic-studies.org/{}", href);
                    //println!("{}", full_address);

                    let link = client.get(&full_address).send().await.unwrap();
                    let body = link.text().await.unwrap();
                    let document = Html::parse_document(&body);
                    let div_selector = Selector::parse("div[id=\"right-side\"]").unwrap();
                    let right_side = document.select(&div_selector);

                    for html_product in right_side {
                        let author = html_product
                            .select(&Selector::parse(".work_author").unwrap())
                            .next()
                            .and_then(|span| Some(span.text().collect::<String>())).as_deref()
                            .map(str::to_owned);
                        let title = html_product
                            .select(&Selector::parse(".work_title").unwrap())
                            .next()
                            .and_then(|span| Some(span.text().collect::<String>())).as_deref()
                            .map(str::to_owned);
                        let english = html_product
                            .select(&Selector::parse("p").unwrap())
                            .next()
                            .and_then(|strong| strong.next_sibling().text().collect::<String>()).as_deref()
                            .map(str::to_owned);

                            


                        let works = Work {
                            author,
                            title,
                            english,
                            /*original,
                            domains,
                            r#type,
                            translator,
                            translated_from,
                            date,
                            publication_type,
                            author_editor,
                            title,
                            r#in,
                            published,
                            volumes,
                            pages,*/
                        };
                        work.push(works);

                        let mut csv_writer = Writer::from_path("products.csv").unwrap();
                        csv_writer.write_record(&["Author", "Title", "English"/*, "Original", "Domains", "Type", "Translator", "Translated from",
                         "Date", "Publication type", "Author/Editor", "Title", "In", "Published", "Volumes", "Pages"*/]).unwrap();
                        

                        for product in &work {
                            let author = product.author.clone().unwrap();
                            let title = product.title.clone().unwrap();
                            let english = product.english.clone().unwrap();
                            csv_writer.write_record(&[author, title, english]).unwrap();
                            
                            csv_writer.flush().unwrap();
                        }
                    }
                }
            }
        }
    }
}
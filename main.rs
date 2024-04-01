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
    original: Option<String>,
    domains: Option<String>,
    rtype: Option<String>,
    translator: Option<String>,
    translated_from: Option<String>,
    date: Option<String>,
    publication_type: Option<String>,
    author_editor: Option<String>,
    title_b: Option<String>,
    rin: Option<String>,
    published: Option<String>,
    volume: Option<String>,
    pages: Option<String>,
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
                            .select(&Selector::parse("strong").unwrap())
                            .filter(|strong| {
                                strong.text().any(|text| text.contains("English:"))
                            })
                            .next()
                            .and_then(|strong| {
                                strong
                                .next_sibling()
                                .and_then(|node| node.value().as_text())
                                .map(|text| text.trim().to_owned())
                            });

                            let original = html_product
                            .select(&Selector::parse("strong").unwrap())
                            .filter(|strong| {
                                strong.text().any(|text| text.contains("Original:"))
                            })
                            .next()
                            .and_then(|strong| {
                                strong
                                .next_sibling()
                                .and_then(|node| node.value().as_text())
                                .map(|text| text.trim().to_owned())
                            });

                            let domains = html_product
                            .select(&Selector::parse("strong").unwrap())
                            .filter(|strong| {
                                strong.text().any(|text| text.contains("Original:"))
                            })
                            .next()
                            .and_then(|strong| {
                                strong
                                .next_sibling()
                                .and_then(|node| node.value().as_text())
                                .map(|text| text.trim().to_owned())
                            });

                            let rtype = html_product
                            .select(&Selector::parse("b").unwrap())
                            .filter(|strong| {
                                strong.text().any(|text| text.contains("Type:"))
                            })
                            .next()
                            .and_then(|strong| {
                                strong
                                .next_sibling()
                                .and_then(|node| node.value().as_text())
                                .map(|text| text.trim().to_owned())
                            });

                            let translator = html_product
                            .select(&Selector::parse("b").unwrap())
                            .filter(|strong| {
                                strong.text().any(|text| text.contains("Translator:"))
                            })
                            .next()
                            .and_then(|strong| {
                                strong
                                .next_sibling()
                                .and_then(|node| node.value().as_text())
                                .map(|text| text.trim().to_owned())
                            });

                            let translated_from = html_product
                            .select(&Selector::parse("b").unwrap())
                            .filter(|strong| {
                                strong.text().any(|text| text.contains("Translated from:"))
                            })
                            .next()
                            .and_then(|strong| {
                                strong
                                .next_sibling()
                                .and_then(|node| node.value().as_text())
                                .map(|text| text.trim().to_owned())
                            });

                            let date = html_product
                            .select(&Selector::parse("b").unwrap())
                            .filter(|strong| {
                                strong.text().any(|text| text.contains("Date:"))
                            })
                            .next()
                            .and_then(|strong| {
                                strong
                                .next_sibling()
                                .and_then(|node| node.value().as_text())
                                .map(|text| text.trim().to_owned())
                            });

                            let publication_type = html_product
                            .select(&Selector::parse("b").unwrap())
                            .filter(|strong| {
                                strong.text().any(|text| text.contains("Publication type:"))
                            })
                            .next()
                            .and_then(|strong| {
                                strong
                                .next_sibling()
                                .and_then(|node| node.value().as_text())
                                .map(|text| text.trim().to_owned())
                            });

                            let author_editor = html_product
                            .select(&Selector::parse("b").unwrap())
                            .filter(|strong| strong.text().any(|text| text.contains("Author/Editor:")))
                            .next()
                            .and_then(|strong| {
                                strong
                                    .next_sibling()
                                    .and_then(|node| node.value().as_text())
                                    .map(|text| text.trim().to_owned())
                            })
                            .unwrap_or_else(|| "Pas d'information trouvé".to_owned());

                            let title_b = html_product
                            .select(&Selector::parse("b").unwrap())
                            .filter(|strong| {
                                strong.text().any(|text| text.contains("Title:"))
                            })
                            .next()
                            .and_then(|strong| {
                                strong
                                .next_sibling()
                                .and_then(|node| node.value().as_text())
                                .map(|text| text.trim().to_owned())
                            });

                            let rin = html_product
                            .select(&Selector::parse("b").unwrap())
                            .filter(|strong| strong.text().any(|text| text.contains("in:")))
                            .next()
                            .and_then(|strong| {
                                strong
                                    .next_sibling()
                                    .and_then(|node| node.value().as_text())
                                    .map(|text| text.trim().to_owned())
                            })
                            .unwrap_or_else(|| "Pas d'information trouvé".to_owned());

                            let published = html_product
                            .select(&Selector::parse("b").unwrap())
                            .filter(|strong| strong.text().any(|text| text.contains("Published:")))
                            .next()
                            .and_then(|strong| {
                                strong
                                    .next_sibling()
                                    .and_then(|node| node.value().as_text())
                                    .map(|text| text.trim().to_owned())
                            })
                            .unwrap_or_else(|| "Pas d'information trouvé".to_owned());

                            let volume = html_product
                            .select(&Selector::parse("b").unwrap())
                            .filter(|strong| strong.text().any(|text| text.contains("Volume:")))
                            .next()
                            .and_then(|strong| {
                                strong
                                    .next_sibling()
                                    .and_then(|node| node.value().as_text())
                                    .map(|text| text.trim().to_owned())
                            })
                            .unwrap_or_else(|| "Pas d'information trouvé".to_owned());

                            let pages = html_product
                            .select(&Selector::parse("b").unwrap())
                            .filter(|strong| strong.text().any(|text| text.contains("Pages:")))
                            .next()
                            .and_then(|strong| {
                                strong
                                    .next_sibling()
                                    .and_then(|node| node.value().as_text())
                                    .map(|text| text.trim().to_owned())
                            })
                            .unwrap_or_else(|| "Pas d'information trouvé".to_owned());


                        let works = Work {
                            author,
                            title,
                            english,
                            original,
                            domains,
                            rtype,
                            translator,
                            translated_from,
                            date,
                            publication_type,
                            author_editor: Some(author_editor),
                            title_b,
                            rin: Some(rin),
                            published: Some(published),
                            volume: Some(volume),
                            pages: Some(pages),
                        };
                        work.push(works);

                        let mut csv_writer = Writer::from_path("products.csv").unwrap();
                        csv_writer.write_record(&["Author", "Title", "English", "Original", "Domains", "Type", "Translator", "Translated from",
                         "Date", "Publication type", "Author/Editor", "Title", "In", "Published", "Volume", "Pages"]).unwrap();
                        

                        for product in &work {
                            let author = product.author.clone().unwrap();
                            let title = product.title.clone().unwrap();
                            let english = product.english.clone().unwrap();
                            let original = product.original.clone().unwrap();
                            let domains = product.domains.clone().unwrap();
                            let rtype = product.rtype.clone().unwrap();
                            let translator = product.translator.clone().unwrap();
                            let translated_from = product.translated_from.clone().unwrap();
                            let date = product.date.clone().unwrap();
                            let publication_type = product.publication_type.clone().unwrap();
                            let author_editor = product.author_editor.clone().unwrap();
                            let title_b = product.title.clone().unwrap();
                            let rin = product.rin.clone().unwrap();
                            let published = product.published.clone().unwrap();
                            let volume = product.volume.clone().unwrap();
                            let pages = product.pages.clone().unwrap();
                            csv_writer.write_record(&[author, title, english, original, domains, rtype, translator, translated_from,
                            date, publication_type, author_editor, title_b, rin, published, volume, pages]).unwrap();
                            csv_writer.flush().unwrap();
                        }
                    }
                }
            }
        }
    }
}
}

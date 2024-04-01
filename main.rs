//Importation des bibliothèques (crates) à utiliser dans le code.
use reqwest::Client;
use scraper::{Html, Selector};
use std::collections::HashSet;
use tokio;
use csv::Writer;
//use ego_tree::NodeRef;

//Mise en place de la fonction asynchrone
#[tokio::main]
async fn main() {
    data().await;
}

//Création d'une structure Work pour la récupération des données.
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

//Début de la fonction asychrone data
async fn data() {
    let mut work: Vec<Work> = Vec::new();
    let mut unique_addresses = HashSet::new();
    let client = Client::new();
    let link = client.get("https://www.graeco-arabic-studies.org/texts.html").send().await.unwrap(); //--> Envoie d'une requête et récupération du code source à cette adresse

    let body = link.text().await.unwrap();
    let document = Html::parse_document(&body);
    let li_selector = Selector::parse("li.l3.l3-c2").unwrap();//--> Transformation du code source en format texte et selection de la liste d'items souhaité

    for li in document.select(&li_selector) {
        let a_li = Selector::parse("a").unwrap();
        for a in li.select(&a_li) {
            let href = a.value().attr("href").unwrap_or("");
            if href.starts_with("single-text/") {
                if unique_addresses.insert(href) {
                    let full_address = format!("https://www.graeco-arabic-studies.org/{}", href);//--> Récupération de parties d'adresses et reconstitution des adresses
                    //println!("{}", full_address);

                    let link = client.get(&full_address).send().await.unwrap();
                    let body = link.text().await.unwrap();
                    let document = Html::parse_document(&body);
                    let div_selector = Selector::parse("div[id=\"right-side\"]").unwrap();
                    let right_side = document.select(&div_selector);//--> Envoie d'une requête et récupération du code source aux adresses qui ont été reconstitué, selection de tous les div ayant un id=right-side

                    for html_data in right_side {
                        //--> Recherche des éléments CSS ".work_author" dans le code source et récupération des données
                        let author = html_data
                            .select(&Selector::parse(".work_author").unwrap())
                            .next()
                            .and_then(|span| Some(span.text().collect::<String>())).as_deref()
                            .map(str::to_owned);

                        //--> Recherche des éléments CSS ".work_title" dans le code source et récupération des données
                        let title = html_data
                            .select(&Selector::parse(".work_title").unwrap())
                            .next()
                            .and_then(|span| Some(span.text().collect::<String>())).as_deref()
                            .map(str::to_owned);

                        //--> Recherche des balises "strong" contenant le texte "English:" dans le code source et récupération des données se trouvant tout juste à la suite
                        let english = html_data
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

                            //--> Recherche des balises "strong" contenant le texte "Original:" dans le code source et récupération des données se trouvant tout juste à la suite
                            let original = html_data
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

                            //--> Recherche des balises "strong" contenant le texte "Domains:" dans le code source et récupération des données se trouvant tout juste à la suite
                            let domains = html_data
                            .select(&Selector::parse("b").unwrap())
                            .filter(|strong| {
                                strong.text().any(|text| text.contains("Domains:"))
                            })
                            .next()
                            .and_then(|strong| {
                                strong
                                .next_sibling()
                                .and_then(|node| node.value().as_text())
                                .map(|text| text.trim().to_owned())
                            });

                            //--> Recherche des balises "b" contenant le texte "Type:" dans le code source et récupération des données se trouvant tout juste à la suite
                            let rtype = html_data
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

                            //--> Recherche des balises "b" contenant le texte "Translator:" dans le code source et récupération des données se trouvant tout juste à la suite
                            let translator = html_data
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

                            //--> Recherche des balises "b" contenant le texte "Translated from:" dans le code source et récupération des données se trouvant tout juste à la suite
                            let translated_from = html_data
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

                            //--> Recherche des balises "b" contenant le texte "Date:" dans le code source et récupération des données se trouvant tout juste à la suite
                            let date = html_data
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

                            //--> Recherche des balises "b" contenant le texte "Publication type:" dans le code source et récupération des données se trouvant tout juste à la suite
                            let publication_type = html_data
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

                            //--> Recherche des balises "b" contenant le texte "Author/Editor:" dans le code source et récupération des données se trouvant tout juste à la suite sinon, affichage du message "Pas d'information trouvé"
                            let author_editor = html_data
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

                            //--> Recherche des balises "b" contenant le texte "Title:" dans le code source et récupération des données se trouvant tout juste à la suite
                            let title_b = html_data
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

                            //--> Recherche des balises "b" contenant le texte "in:" dans le code source et récupération des données se trouvant tout juste à la suite sinon, affichage du message "Pas d'information trouvé"
                            let rin = html_data
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

                            //--> Recherche des balises "b" contenant le texte "Published:" dans le code source et récupération des données se trouvant tout juste à la suite sinon, affichage du message "Pas d'information trouvé"
                            let published = html_data
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

                            //--> Recherche des balises "b" contenant le texte "Volume:" dans le code source et récupération des données se trouvant tout juste à la suite sinon, affichage du message "Pas d'information trouvé"
                            let volume = html_data
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

                            //--> Recherche des balises "b" contenant le texte "Pages:" dans le code source et récupération des données se trouvant tout juste à la suite sinon, affichage du message "Pas d'information trouvé"
                            let pages = html_data
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
                        work.push(works); //--> Association des variables créés ci-dessus avec les branches de la structure "Work" à l'aide d'une nouvelle instance "works" et stockage de celle-ci dans le vecteur "work"

                        let mut csv_writer = Writer::from_path("products.csv").unwrap();
                        csv_writer.write_record(&["Author", "Title", "English", "Original", "Domains", "Type", "Translator", "Translated from",
                         "Date", "Publication type", "Author/Editor", "Title", "In", "Published", "Volume", "Pages"]).unwrap();//--> Création d'un nouveau fichier nommé products.csv et écriture des titres des colonnes
                        

                        for data in &work {
                            let author = data.author.clone().unwrap();
                            let title = data.title.clone().unwrap();
                            let english = data.english.clone().unwrap();
                            let original = data.original.clone().unwrap();
                            let domains = data.domains.clone().unwrap();
                            let rtype = data.rtype.clone().unwrap();
                            let translator = data.translator.clone().unwrap();
                            let translated_from = data.translated_from.clone().unwrap();
                            let date = data.date.clone().unwrap();
                            let publication_type = data.publication_type.clone().unwrap();
                            let author_editor = data.author_editor.clone().unwrap();
                            let title_b = data.title.clone().unwrap();
                            let rin = data.rin.clone().unwrap();
                            let published = data.published.clone().unwrap();
                            let volume = data.volume.clone().unwrap();
                            let pages = data.pages.clone().unwrap();
                            csv_writer.write_record(&[author, title, english, original, domains, rtype, translator, translated_from,
                            date, publication_type, author_editor, title_b, rin, published, volume, pages]).unwrap();
                            csv_writer.flush().unwrap();//--> Itération sur chaque valeur "data" contenu dans la référence du vecteur work et écriture de celles-ci dans le fichier .csv
                        }
                    }
                }
            }
        }
    }
}

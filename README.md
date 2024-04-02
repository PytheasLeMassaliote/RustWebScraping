# RustWebScraping
Dans le cadre d'une recherche portant sur le mouvement de traduction Greco-Arabe (pour ma conjointe),
et au vu du peu de données structurées existant à cet endroit,
j'ai décidé de me consacrer à la réalisation d'un projet de "web scrapping", 
avec l'intention d'approfondir mes connaissances en front-end (structure HTML et CSS), ainsi que de me pousser concrètement à la pratique du langage Rust.

L'objectif du programme est d'extraire des données à partir du site https://www.graeco-arabic-studies.org/texts.html et plus précisément,
toutes les informations relatives aux traductions Grec/Latin ----> Arabe.

A l'aide de différentes sources d'informations, j'ai pu réaliser mon script au bout de plusieurs essais,
qui m'ont beaucoup appris.

Crates utilisées : 
- tokio = { version = "1", features = ["full"] } : "tokio" permet de créer des fonctions asynchrones
- reqwest = "0.11.24" : "reqwest" permet d'envoyer des requêtes de récupération du code source d'une page web via son adresse //http
- scraper = "0.12.0" : "scraper" permet d'analyser le code source de la page web et de sélectionner les données souhaitées.
- csv = "1.3.0" : "csv" permet le stockage de toutes ces données dans un fichier .csv
 
Support de travail :
- https://www.graeco-arabic-studies.org

Sources :
- https://scrape-it.cloud/blog/web-scraping-with-rust#parsing-html-document
- https://www.scrapingdog.com/blog/web-scraping-with-rust/
- https://jimskapt.github.io/rust-book-fr/
- Youtube (beaucoup de tutos en web scraping différents)
- https://docs.rs (recense énormément de crates)

Outils :
- VisualCode (Add-on --> Even Better TOML, rust-analyzer)






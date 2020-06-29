use serde_json::{Result,Value};
use std::fs::File;

fn reddit_json() -> Result<()> {
    let Data = r#"{
        "data": {"modhash": "","dist":25},
        "selftext": "Hello guys, last year I had the idea to do a Hacker News back-end with GraphQL and Datomic, well I left it behind, but with all the recent covid situation I've been digging some old stuff to finish.\n\nI ended adding a front-end, using re-frame for the first time.\n\nMy idea wasn't to do a guide step by step, but mostly an overview of the project, since I didn't find many \"full-stack\" Clojure projects with those libraries from the title, and Datomic, I thought it would be interesting to share. Any feedback, good or bad, is more welcome.\n\n[https://www.giovanialtelino.com/project/hacker-news-graphql/](https://www.giovanialtelino.com/project/hacker-news-graphql/)\n\nBack-end:\n\n[https://github.com/giovanialtelino/hackernews-lacinia-datomic](https://github.com/giovanialtelino/hackernews-lacinia-datomic)\n\nFront-end:\n\n[https://github.com/giovanialtelino/hackernews-reframe](https://github.com/giovanialtelino/hackernews-reframe)",
        "author_name": "t2_nna46",
        "title":"Hacker News with Datomic, Lacinia, re-frame and GraphQL"
    }"#;
    let v: Value = serde_json::from_str(Data)?;
    let mut JsonFile = File::create("data.txt").expect("create failed");
    println!("{:?}", Data);

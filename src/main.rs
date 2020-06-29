use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct reddit_json {
    id: i32,
    dist: i32,
    selftext: String,
    author_fullname: String,
    title: String,
}

fn main() -> Result<()> {
    let conn = Connection::open("reddit.db")?;

    /*
    conn.execute(

        "CREATE TABLE reddit (
            id	               INTEGER PRIMARY KEY AUTOINCREMENT,
            dist	           INTEGER NOT NULL,
            selftext	       TEXT NOT NULL,
            author_fullname    TEXT NOT NULL,
            title	           TEXT NOT NULL UNIQUE
        )",
        params![],
    )?;

    */

    let data = reddit_json {
        id: 0,
        dist: 25,
        selftext: "Hello guys, last year I had the idea to do a Hacker News back-end with GraphQL and Datomic, well I left it behind, but with all the recent covid situation I've been digging some old stuff to finish.\n\nI ended adding a front-end, using re-frame for the first time.\n\nMy idea wasn't to do a guide step by step, but mostly an overview of the project, since I didn't find many \"full-stack\" Clojure projects with those libraries from the title, and Datomic, I thought it would be interesting to share. Any feedback, good or bad, is more welcome.\n\n[https://www.giovanialtelino.com/project/hacker-news-graphql/](https://www.giovanialtelino.com/project/hacker-news-graphql/)\n\nBack-end:\n\n[https://github.com/giovanialtelino/hackernews-lacinia-datomic](https://github.com/giovanialtelino/hackernews-lacinia-datomic)\n\nFront-end:\n\n[https://github.com/giovanialtelino/hackernews-reframe](https://github.com/giovanialtelino/hackernews-reframe)".to_string(),
        author_fullname: "t2_nna46".to_string(),
        title: "Hacker News with Datomic, Lacinia, re-frame and GraphQL".to_string()
    };

    conn.execute(
        "INSERT or REPLACE INTO reddit (dist,selftext,author_fullname,title) values(?1, ?2, ?3, ?4)",
        params![data.dist,data.selftext,data.author_fullname,data.title],
    )?;

    let mut stmt = conn.prepare("SELECT id, dist, selftext, author_fullname, title FROM reddit")?;
    let reddit_iter = stmt.query_map(params![], |row| {
        Ok(reddit_json {
            id: row.get(0)?,
            dist: row.get(1)?,
            selftext: row.get(2)?,
            author_fullname: row.get(3)?,
            title: row.get(4)?,
        })
    })?;

    for reddit in reddit_iter {
        println!("Found data {:?}", reddit.unwrap());
    }
    Ok(())
}
extern crate serde;
extern crate serde_json;
extern crate reqwest;
extern crate serde_derive;
extern crate rusqlite;

use anyhow::Result;
//use std::string;
use std::fs::{File, create_dir};
use std::io::Read;
//use reqwest::{IntoUrl, Client};
use std::io::prelude::*;
//use std::error::Error;
//use std::io::BufReader;
//use std::path::Path;
use serde::Deserialize;
use rusqlite::{params, Connection};
//use std::env::args;

  /* struct RedditJson {
        id: i32,
        data: {dist:i32, title:String},
        selftext: String,
        author_fullname: String,
    }
    */


/*
#[derive(Debug,Deserialize)]
struct RedditJson {
    data: Datac,
}

struct Datac {
    children: Vec<Datas>,
}

struct Datas {

    selftext: String, 
    author_fullname: String, 
    title: String,
}

fn read_json_from_file<P: AsRef<Path>>(path: P) -> Result<RedditJson, Box<Error>> {
    //open the file in read only
    let reader = BufReader::new(file);
    let mut file = File::open(path)?;
    //Read the JSON file as an instance of RedditJson struct
    let read_file = serde_json::from_reader(reader)?;

    
    Ok(read_file)
}
*/

// creating struct for redditjson
#[derive(Deserialize, Debug)]
struct RedditJson {
    data:  Data,
}

impl RedditJson {
    fn subreddit_get_request(subreddits:Vec<String>) -> Vec<RedditJson> {

        let mut subredditjsons: Vec<RedditJson> = Vec::new();
        for i in subreddits.iter() {        
            let sites = "https://www.reddit.com/r/".to_owned() + i + ".json";
            let mut body = String::new();
            let _json_body = reqwest::blocking::get(&sites).unwrap().read_to_string(&mut body);
            let datas = serde_json::from_str::<RedditJson>(&body).unwrap();
            subredditjsons.push(datas);
        }
        //println!("{:?}", subredditjsons);
        subredditjsons
    }

    fn add_to_db(new: &Vec<RedditJson>, conn: &Connection) -> Result<()> {
        
        for j in new {
            for k in &j.data.children {
            conn.execute(
                "INSERT or REPLACE INTO json (title,author_fullname,url) values(?1, ?2, ?3)",
                params![k.data.title,k.data.author_fullname,k.data.url],
            )?;

            }   
        }
            Ok(())
    }
}

// creating struct for `vec` i.e. list 
#[derive(Deserialize, Debug)]
struct Data {
    children: Vec<Children>,
}

// creating a struct 
#[derive(Deserialize, Debug)]
struct Children {
    data: Datas,
}

//creating struct for `Datas`
#[derive(Deserialize, Debug)]
struct Datas {
    title: String,
    author_fullname: String,
    url: String,
}

impl Datas {
    fn get_all_from_db(conn: &Connection) -> Result<Vec<Datas>> {
        let mut stmt = conn.prepare("SELECT ID, title, author_fullname, url FROM json")?;
        let reddit_iter = stmt.query_map(params![], |row| {
            Ok(Datas {
                title: row.get(1)?,
                author_fullname: row.get(2)?,
                url: row.get(3)?,
            })
        })?;

        let mut db_datas: Vec<Datas> = Vec::new();
        for data in reddit_iter {
            db_datas.push(data.unwrap());
        }
        
        Ok(db_datas)
    }
}


#[derive(Deserialize, Debug)]
struct UrlId {
    Id: i32,
    url: String,
}



pub fn create_db(conn: &Connection) -> Result<()> {
    
    conn.execute(
        "CREATE TABLE json (
            ID                 INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,             
            title	           TEXT NOT NULL,
            author_fullname    TEXT NOT NULL,
            url	               TEXT NOT NULL UNIQUE
        )",
        params![],
    )?;

    Ok(())
}
/*
pub fn subreddit_get_request (subreddits:Vec<String>) -> Result<()> {
    for i in subreddits.iter() {
        
        let sites = "https://www.reddit.com/r/".to_owned() + i + ".json";
        let read_file = reqwest::blocking::get(&sites)?.text()?;
        let datas = serde_json::from_str::<RedditJson>(&read_file).unwrap();
    }

    Ok(())
}
*/


pub fn url_get_request(id:String, url:String) -> Result<()> {

    let client = reqwest::blocking::Client::new();
    let mut res = client.get(&url).send().unwrap(); 

    let mut body = Vec::new();
    res.read_to_end(&mut body)?;

    let  paths = format!("{}.html", &id);
    let  path = format!("{}", &id);
    let  file_path = format!("{}/{}",&path, &paths);

    let mut _html_dir = create_dir(path)?;
    let mut html_file = File::create(file_path)?;
    html_file.write_all(&body.as_mut())?;
    println!("Status: {}", res.status());

    Ok(())
} 

//main function
fn main() -> Result<()> {


    // Created a database named reddit.db
    let conn = Connection::open_in_memory().unwrap();
    
    create_db(&conn)?;
    
    /*
    conn.execute(
        "CREATE TABLE json (
            ID                 INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,             
            title	           TEXT NOT NULL,
            author_fullname    TEXT NOT NULL,
            url	               TEXT NOT NULL UNIQUE
        )",
        params![],
    )?;
    */ 
    
    //HTTP get request
    let mut subreddits: Vec<String>= vec!["Clojure".to_string()];
    let mut new = RedditJson::subreddit_get_request(subreddits);

    //for i in subreddits.iter() {

        //let sites = "https://www.reddit.com/r/".to_owned() + i + ".json";
        //let read_file = reqwest::blocking::get(&sites)?.text()?;
        //let datas = serde_json::from_str::<RedditJson>(&read_file).unwrap();
        //for j in &datas.data.children {
        //    println!("{:?}", j );
        //}
        //for j in &datas.data.children {
        //    conn.execute(
        //        "INSERT or REPLACE INTO json (title,author_fullname,url) values(?1, ?2, ?3)",
        //        params![j.data.title,j.data.author_fullname,j.data.url],
        //    )?;
                /*
                let mut stmt = conn.prepare("SELECT ID, title, author_fullname, url FROM json")?;
                let reddit_iter = stmt.query_map(params![], |row| {
                    Ok(Datas {
                        title: row.get(1)?,
                        author_fullname: row.get(2)?,
                        url: row.get(3)?,
                    })
                })?;
                */    
                
        RedditJson::add_to_db(&new, &conn)?;
        Datas::get_all_from_db(&conn)?;

        let mut url_stmt  = conn.prepare("SELECT ID, url FROM json")?;
        let url_reddit_iter = url_stmt.query_map(params![], |row| {
            Ok(UrlId {
                Id: row.get(0)?,
                url: row.get(1)?,
            })
        })?;
    

        let mut url_names = Vec::new();
        for name_result in url_reddit_iter {
            url_names.push(name_result?);
        }



        let mut count = 0;
        for links in url_names.iter() {
            let url = links.url.to_string();
            let id = links.Id.to_string();
            if &url == "https://www.linkedin.com/jobs/view/1938385901/" {
                continue;
            }

            url_get_request(id, url).expect("No problem");
            /*
            let client = reqwest::blocking::Client::new();
            let mut res = client.get(&url).send().unwrap(); 

            let mut body = Vec::new();
            res.read_to_end(&mut body)?;

            let mut paths = format!("{}.html", &id.to_string());
            let mut path = format!("{}", &id.to_string());
            let mut file_path = format!("{}/{}",&path, &paths);

            let mut htmlDir = create_dir(path)?;
            let mut htmlFile = File::create(file_path)?;
            htmlFile.write_all(&body.as_mut())?;
            println!("Status: {}", res.status());
            */
            count += 1;
            println!("{}", count);   
            
        }

 
    //let read_file = reqwest::blocking::get("https://www.reddit.com/r/Clojure.json")?.text()?; 
    
    //using serde_json 'from_str'
    //let datas = serde_json::from_str::<RedditJson>(&read_file).unwrap();
    
    //printing the fields 

    //let v = serde_json::from_reader(read_file)?;
    //println!("{}", v["data"]["children"][0]["data"]["selftext"]);

    //called read_json_from_file function and passed as struct
    //let read_file: RedditJson = read_json_from_file("reddit.json").unwrap();

    /*
    let data = RedditJson {
        id: 0,
        dist: 25,
        selftext: "Hello guys, last year I had the idea to do a Hacker News back-end with GraphQL and Datomic, well I left it behind, but with all the recent covid situation I've been digging some old stuff to finish.\n\nI ended adding a front-end, using re-frame for the first time.\n\nMy idea wasn't to do a guide step by step, but mostly an overview of the project, since I didn't find many \"full-stack\" Clojure projects with those libraries from the title, and Datomic, I thought it would be interesting to share. Any feedback, good or bad, is more welcome.\n\n[https://www.giovanialtelino.com/project/hacker-news-graphql/](https://www.giovanialtelino.com/project/hacker-news-graphql/)\n\nBack-end:\n\n[https://github.com/giovanialtelino/hackernews-lacinia-datomic](https://github.com/giovanialtelino/hackernews-lacinia-datomic)\n\nFront-end:\n\n[https://github.com/giovanialtelino/hackernews-reframe](https://github.com/giovanialtelino/hackernews-reframe)".to_string(),
        author_fullname: "t2_nna46".to_string(),
        title: "Hacker News with Datomic, Lacinia, re-frame and GraphQL".to_string()
    };

    */
    /*
    //Inserted the read-file data in created database
    for i in &datas.data.children {
        conn.execute(
            "INSERT or REPLACE INTO json (selftext,author_fullname,title) values(?1, ?2, ?3)",
            params![i.data.selftext,i.data.author_fullname,i.data.title],
        )?;
        
        let mut stmt = conn.prepare("SELECT selftext, author_fullname, title FROM json")?;
        let reddit_iter = stmt.query_map(params![], |row| {
            Ok(Datas {
                selftext: row.get(0)?,
                author_fullname: row.get(1)?,
                title: row.get(2)?,
            })
        })?;
    }
    */
    

    Ok(())
}


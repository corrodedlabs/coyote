extern crate serde;
extern crate serde_json;
extern crate reqwest;
extern crate serde_derive;
extern crate rusqlite;
extern crate clap;
extern crate url;

use anyhow::Result;
//use std::string;
use std::process;
use std::time::Duration;
use reqwest::Error as errs;
use std::fs::{File, create_dir};
use url::{Url, Host, ParseError, Position};
use std::io;
use std::io::prelude::*;
//use std::error::Error;
//use std::io::BufReader;
//use std::path::Path;
use serde::Deserialize;
use rusqlite::{params, Connection};
use std::env;
use std::io::ErrorKind;
use clap::{Arg, App, SubCommand};

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
    //sending request and pushing it in the vector
    fn subreddit_get_request(subreddits: &Vec<String>) -> Vec<RedditJson> {
    
        let mut subredditjsons: Vec<RedditJson> = Vec::new();
        for i in subreddits.iter() {
            let sites = "https://www.reddit.com/r/".to_owned() + &i + ".json";
            let mut body = String::new();
            let _json_body = reqwest::blocking::get(&sites).unwrap().read_to_string(&mut body);
            let datas = serde_json::from_str::<RedditJson>(&body).unwrap();
            subredditjsons.push(datas);
        }
        println!("It might take few seconds!!!");
        //println!("{:?}", subredditjsons);
        subredditjsons
    }

    //Adding the vector elements in the database
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
    //Adding database data in a vector
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

#[derive(Debug)]
struct ErrorUrlId {
    id: i32,
    url: String,
}

impl ErrorUrlId {
    //passing url requests through `id`and `url`
    //And adding the error url in a vector
    pub fn url_get_request(id: String, url: String, errors: &mut std::vec::Vec<ErrorUrlId>) -> Result<()> {

        let client = reqwest::blocking::Client::new();
        let mut res = client.get(&url).send();
        let x = match res {
            Ok(mut res) => {
                /*
                let mut body = Vec::new();
                res.read_to_end(&mut body).expect("error: Cannot read till the end");
    
                let  paths = format!("{}.html", &id);
                let  path = format!("{}", &id);
                let  file_path = format!("{}/{}",&path, &paths);
    
                let mut _html_dir = create_dir(path).expect("error: Cannot create directory");
                let mut html_file = File::create(file_path).expect("error: cannot create the file");
                html_file.write_all(&body.as_mut()).expect("error: cannot write the body");
                */
                read_file (&id, &mut res);
                println!("Status: {}", res.status());            
            },
    
            Err(error) => match error.is_timeout() {
                true => {
                    let ids = id.parse::<i32>().unwrap(); //changing string into an integer`<i32>`
                    errors.push(ErrorUrlId {
                        id: ids,
                        url,
                    });
                },
                false => {
                    println!("other error");
                },
    
            },
        };   
        Ok(())
    }

    //viewing errorlist
    //Adding errorlist from db to vector of `ErrorUrlId`  
    pub fn errorlist_view_db(conn: &Connection) -> Vec<ErrorUrlId> {
        let mut url_stmt  = conn.prepare("SELECT id, url FROM errorurlid").expect("cannot get seprated");
        let error_url_reddit_iter = url_stmt.query_map(params![], |row| {
        Ok(ErrorUrlId {
            id: row.get(0).expect("get error"),
            url: row.get(1).expect("get error"),
            })
        }).expect("error: cant seprate");

        let mut error_url_names = Vec::new();
        for names in error_url_reddit_iter {
            error_url_names.push(names.unwrap());
        }

    error_url_names

    }
    //retry requesting the error prone  url
    pub fn retry_error_list(errorurls: Vec<ErrorUrlId>) -> Result<()> {
        let mut count = 0;
        for urlnames in errorurls.iter() {
            let url = urlnames.url.to_string();
            let id = urlnames.id.to_string();

            let mut issuelisturl = Url::parse(&url.to_string());
            if issuelisturl== Err(ParseError::RelativeUrlWithoutBase)  {
                //let mut error = url.clone();
                //errorlist.push(error);
                continue;
            }
            
            let client = reqwest::blocking::Client::new();
            let mut res = client.get(&url).send().unwrap();
            read_file (&id, &mut res);
            count += 1;
            println!("{}\n{}",&res.status(), count);
        }
        Ok(())
    }
}

//Reading the file from the response of the `url` 
pub fn read_file (ids: &String, response: &mut reqwest::blocking::Response) -> File {
    let mut body = Vec::new();
    response.read_to_end(&mut body).expect("error: Cannot read till the end");

    let  paths = format!("{}.html", &ids);
    let  path = format!("{}", &ids);
    let  file_path = format!("{}/{}",&path, &paths);

    let mut _html_dir = create_dir(path).expect("error: Cannot create directory");
    let mut html_file = File::create(file_path).expect("error: cannot create the file");
    html_file.write_all(&body.as_mut()).expect("error: cannot write the body");
    
    html_file
}

#[derive(Deserialize, Debug)]
struct UrlId {
    Id: i32,
    url: String,
}

impl UrlId {
    //fetching the data by sending request
    //saving the error url in a vector
    // and adding the data in a new table  
    pub fn fetchdata (conn: &Connection) -> Result<()> {
        let mut url_stmt  = conn.prepare("SELECT ID, url FROM json").expect("cannot get seprated");
        let url_reddit_iter = url_stmt.query_map(params![], |row| {
        Ok(UrlId {
            Id: row.get(0).expect("get error"),
            url: row.get(1).expect("get error"),
            })
        }).expect("error: cant seprate");

        let mut url_names = Vec::new();
        for name_result in url_reddit_iter {
            url_names.push(name_result.unwrap());
        }

        let mut count = 0;
        let mut errors: Vec<ErrorUrlId> = Vec::new();

        for links in url_names.iter() {
            let url = links.url.to_string();
            let id = links.Id.to_string();
            //if &url == "https://www.linkedin.com/jobs/view/1938385901/" {
            //    continue;
            //}
            let mut issuelisturl = Url::parse(&url.to_string());
            if issuelisturl== Err(ParseError::RelativeUrlWithoutBase)  {
                //let mut error = url.clone();
                //errorlist.push(error);
                continue;
            }    

            ErrorUrlId::url_get_request(id, url, &mut errors).expect("error: couldnot get request");

            count += 1;
            println!("{}", count);                   
        }
        
        for j in errors {
            conn.execute(
                "INSERT or REPLACE INTO errorurlid (id,url) values(?1, ?2)",
                params![j.id, j.url],
            )?;

        }
        

        Ok(())
    }


}


//Adding the subreddits in a vector
pub fn addsubredditstring () -> Vec<String> {

    println!("Add subreddit");
    println!("Add whitespace to add more subreddits");
    let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Error reading input");
    let mut subreddit: Vec<String> = input                       //removing whitespace between
        .split_whitespace()                             //string and adding 
        .map(|s| s.parse().expect("parse error")).collect(); //it in a vector

    println!("{:?}", subreddit);
    subreddit
}

//creating database
pub fn create_db(conn: &Connection) -> Result<()> {
    
    conn.execute(
    "CREATE TABLE IF NOT EXISTS json (
            ID                 INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,             
            title	           TEXT NOT NULL,
            author_fullname    TEXT NOT NULL,
            url	               TEXT NOT NULL UNIQUE
        )",
        params![],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS errorurlid (
            id      INTEGER NOT NULL PRIMARY KEY,
            url     TEXT NOT NULL
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


//main function
fn main() -> Result<()> {


    // Created a database named reddit.db
    let conn = Connection::open("json.db").unwrap();
    
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
    

    let mut new;
    let mut subreddit;
    let mut args: Vec<String> = env::args().collect();
    //let mut errorlist: Vec<String> = Vec::new();
    //let mut errorlists: Vec<String> = Vec::new();
    let cmd = &args[1];
    //matching the arguments 
    match &cmd[..] {

        "addsubreddit" => {
            subreddit = addsubredditstring();
            println!("It might take few seconds....");
            new = RedditJson::subreddit_get_request(&subreddit);
            RedditJson::add_to_db(&new, &conn)?;
            Datas::get_all_from_db(&conn)?;  
        },

        "fetchdata" => {
            //new = RedditJson::subreddit_get_request(&addsubredditstring());
            //RedditJson::add_to_db(&new, &conn)?;
            //Datas::get_all_from_db(&conn)?;    

            let x = UrlId::fetchdata(&conn);
            /*
            let mut count = 0;
            for links in x.iter() {
                let url = links.url.to_string();
                let id = links.Id.to_string();
                //if &url == "https://www.linkedin.com/jobs/view/1938385901/" {
                //    continue;
                //}
                let mut issuelisturl = Url::parse(&url.to_string());
                if issuelisturl== Err(ParseError::RelativeUrlWithoutBase)  {
                    let mut error = url.clone();
                    errorlist.push(error);
                    continue;
                }    

                url_get_request(id, url, errorlists)?;

                count += 1;
                println!("{}", count);                   
            }
            */

        },

        "viewerrors" => {
            let x = ErrorUrlId::errorlist_view_db(&conn);
            println!("{:?}\n", x);
        },

        "retryerror" => {
            let retry = ErrorUrlId::errorlist_view_db(&conn);
            ErrorUrlId::retry_error_list(retry)?;
        }

        "help" => {
            let s = "1. Add subreddit => `cargo run addsubreddit`\n\
                        2. Fetch data => `cargo run fetchdata`\n\
                        3. view errors => `cargo run viewerrors`\n\
                        4.Retry errors => `cargo run retryerror`";
            println!("commands:\n{}", s);
        },

        _ => {
            println! ("FuckOff!");
        },
                
    }
    //println!("{:?}", &urlgetrequest);
    //println!("error list: {:?}", &errorlist); 
    //println!("{:?}", &RedditJson::subreddit_get_request(&addsubredditstring()));

    //let mut subreddits: Vec<String>= env::args().collect();
    //let mut subreddits=vec!["clojure".to_string()];
    //let mut new = RedditJson::subreddit_get_request(&subreddits);

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
        /*        
        RedditJson::add_to_db(&RedditJson::subreddit_get_request(&addsubredditstring()), &conn)?;
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
        */

 
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


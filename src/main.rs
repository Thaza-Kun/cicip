#![allow(dead_code)]

use serde::Deserialize;
use std::io::Write;
use std::path::PathBuf;
use std::{fs, io};
use clap::{Args, Parser, Subcommand, ValueEnum};
use ratatui::layout::Rect;
use tweet::TweetItem;
use crate::facebook::Facebook;

mod call_to_action;
mod entities;
mod tweet;
mod user;
mod facebook;

#[derive(ValueEnum, Debug, Clone)]
enum MedSos {
    Twitter,
    Facebook
}

#[derive(Args, Debug)]
struct Read {
    media: MedSos
}
#[derive(Parser, Debug)]
enum Command {
    Read(Read)
}

fn main() {
    match Command::parse() {
        Command::Read(Read {media: m}) => match m {
            MedSos::Twitter => { let tweets = read_tweets(); println!("{} tweets read from archive.", tweets.len())},
            MedSos::Facebook => {read_facebook();}
        }
    };
}

fn read_facebook() -> () {
    let datapath: String = dotenvy::var("FACEBOOK_DATAPATH").expect(
        "`FACEBOOK_DATAPATH` variable in `.env` with the correct formatting. (Use `/` instead of `\\`)",
    );
    let content = fs::read_to_string([&datapath, "posts", "your_posts_1.json"]
        .iter()
        .collect::<PathBuf>())
    .expect("File not found");
    let f: Vec<Facebook> = serde_json::from_str(&content).unwrap();
}

fn read_tweets() -> Vec<TweetItem> {
    let datapath: String = dotenvy::var("TWITTER_DATAPATH").expect(
        "`TWITTER_DATAPATH` variable in `.env` with the correct formatting. (Use `/` instead of `\\`)",
    );
    let content = fs::read_to_string([&datapath, "data", "tweet.js"]
        .iter()
        .collect::<PathBuf>())
    .expect("File not found");
    let lines = &mut content.split("=").collect::<Vec<&str>>();
    lines.remove(0);
    TweetItem::from_json(lines.join("="))
}

// TODO Save thread as markdown
// TODO Graph tweets engagement per time

fn main_() {
    #[allow(non_snake_case)]
    let data = read_tweets();

    let immut_tweets = data
        .clone()
        .iter()
        .map(|a| a.clone())
        .collect::<Vec<TweetItem>>();

    let mut tweets = data
        .clone()
        .iter()
        .map(|a| a.clone())
        .collect::<Vec<TweetItem>>();
    tweets.sort_by(|a, b| b.favorite_count.cmp(&a.favorite_count));
    let threads = tweets
        .iter()
        .filter(|a| a.favorite_count > 10)
        .collect::<Vec<&TweetItem>>();

    let mut idx: usize = 0;
    let mut stdout = io::stdout();
    loop {
        let mut line = String::new();
        println!("{}", threads[idx]);
        println!(
            "Comments: {}",
            threads[idx]
                .get_comments(&threads.iter().map(|&a| a.clone()).collect())
                .len()
        );
        println!(
            "Chain: {}",
            threads[idx]
                .chain_heads(&threads.iter().map(|&a| a.clone()).collect())
                .len()
        );
        println!(
            "[(n)ext/(p)revious/(q)uit] ({}/{}): ",
            idx + 1,
            threads.len()
        );
        let input = io::stdin().read_line(&mut line).unwrap();
        match line.to_lowercase().strip_suffix("\r\n") {
            Some("q") => break,
            Some("n") => { idx += 1; stdout.flush().unwrap(); },
            Some("p") => { idx -= 1; stdout.flush().unwrap(); },
            _ => stdout.flush().unwrap(),
        }
    }
}

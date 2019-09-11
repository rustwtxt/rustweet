//
// rustweet - Copyright (c) 2019 Ben Morrison (gbmor)
// See LICENSE file for detailed license information.
//
use chrono::prelude::*;
use rustwtxt::TweetMap;

use std::collections::BTreeMap;
use std::fs;
use std::process;

use crate::conf;
use crate::ed;

#[derive(Debug, Clone)]
struct Timeline {
    tweets: TweetMap,
}

pub fn tweet() {
    let twtxt_path = &*conf::DATA.path.clone();
    let tweet_body = ed::call();

    let timestamp =
        Utc::now().to_rfc3339_opts(SecondsFormat::Secs, false);

    let tweet_body = format!("{}\t{}", timestamp, tweet_body);

    let current_tweets = match fs::read_to_string(twtxt_path) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Can't read twtxt.txt: {:?}", err);
            process::exit(1);
        }
    };

    let mut line_vec =
        current_tweets.split("\n").collect::<Vec<&str>>();
    let mut trimmed_line_vec = Vec::new();
    line_vec.iter().for_each(|line| {
        if line == &"" {
            return;
        }
        trimmed_line_vec.push(line.to_owned());
    });

    line_vec.push(&tweet_body);
    let new_tweets = trimmed_line_vec.join("\n");

    match fs::write(twtxt_path, new_tweets) {
        Err(err) => {
            eprintln!(
                "Couldn't append new tweet to twtxt.txt: {:?}",
                err
            );
        }
        _ => {
            println!();
            println!("Tweet added!");
            println!();
        }
    }
}

pub fn show() {
    format!(
        "{:#?}",
        Timeline {
            tweets: BTreeMap::new(),
        }
    );
}

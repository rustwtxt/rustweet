//
// rustweet - Copyright (c) 2019 Ben Morrison (gbmor)
// See LICENSE file for detailed license information.
//
use rustwtxt::TweetMap;
use std::collections::BTreeMap;

use crate::ed;

#[derive(Debug, Clone)]
struct Timeline {
    tweets: TweetMap,
}

pub fn tweet() {
    let tweet_body = ed::call();
    dbg!(tweet_body);
}

pub fn show() {
    format!(
        "{:#?}",
        Timeline {
            tweets: BTreeMap::new(),
        }
    );
}

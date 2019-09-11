//
// rustweet - Copyright (c) 2019 Ben Morrison (gbmor)
// See LICENSE file for detailed license information.
//
#[macro_use]
extern crate lazy_static;

use clap;

mod conf;
mod ed;
mod timeline;
mod user;

fn main() {
    let args = clap::App::new("rustwtxt")
        .version(clap::crate_version!())
        .author("Ben Morrison <ben@gbmor.dev>")
        .about("command-line twtxt client")
        .arg(
            clap::Arg::with_name("follow")
                .short("f")
                .long("follow")
                .value_name("URL")
                .help("URL of a user's twtxt.txt file you wish to follow."),
        )
        .arg(
            clap::Arg::with_name("unfollow")
                .short("u")
                .long("unfollow")
                .value_name("NICK")
                .help("Nick of the user you wish to stop following."),
        )
        .subcommand(
            clap::SubCommand::with_name("timeline")
                .about("Displays the followed users' tweets in a timeline."),
        )
        .subcommand(
            clap::SubCommand::with_name("tweet")
                .about("Opens your preferred editor to compose a new tweet."),
        )
        .get_matches();

    if let Some(url) = args.value_of("follow") {
        user::follow(url);
        return;
    } else if let Some(url) = args.value_of("unfollow") {
        user::unfollow(url);
        return;
    }

    match args.subcommand() {
        ("tweet", _args) => {
            timeline::tweet();
            return;
        }
        ("timeline", _args) => {
            timeline::show();
            return;
        }
        (_, _args) => {
            timeline::show();
            return;
        }
    }
}

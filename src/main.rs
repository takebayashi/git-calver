use clap::{App, Arg, ArgMatches, SubCommand};
use git2::Error;
use git2::Repository;

use git_calver::releaser::Releaser;
use git_calver::repo_release::Release;
use git_calver::repo_release::RepositoryWithRelease;

fn cmd_next(_: &ArgMatches) -> Result<(), Error> {
    let repo = Repository::open_from_env().expect("failed to open repository.");
    let releaser = Releaser::new(&repo);
    println!("{}", releaser.next_version());
    Ok(())
}

fn cmd_tag_next(opt: &ArgMatches) -> Result<(), Error> {
    let message = opt.value_of("message").expect("message is required.");
    let repo = Repository::open_from_env().expect("failed to open repository.");
    let releaser = Releaser::new(&repo);
    if !releaser.is_releasable() {
        let latest = repo.find_latest_release().unwrap_or_else(Release::zero);
        eprintln!(
            "commit {} is already tagged: {}",
            latest.commit_id, latest.version
        );
        return Ok(());
    }
    match releaser.bump(message) {
        Ok(v) => println!("{}", v),
        Err(err) => println!("{}", err),
    };
    Ok(())
}

fn cmd_current(_: &ArgMatches) -> Result<(), Error> {
    let repo = Repository::open_from_env().expect("failed to open repository.");
    if let Some(release) = repo.find_latest_release() {
        println!("{}", release.version);
        Ok(())
    } else {
        println!("release not found.");
        Ok(())
    }
}

fn cmd_all(_: &ArgMatches) -> Result<(), Error> {
    let repo = Repository::open_from_env().expect("failed to open repository.");
    for release in repo.find_releases() {
        println!("{}", release.version);
    }
    Ok(())
}

fn main() -> Result<(), Error> {
    let app = App::new("git calver")
        .subcommand(SubCommand::with_name("all").about("get all versions of current tree"))
        .subcommand(SubCommand::with_name("current").about("get latest version of current tree"))
        .subcommand(SubCommand::with_name("next").about("get next version"))
        .subcommand(
            SubCommand::with_name("tag-next")
                .about("tag next version")
                .arg(
                    Arg::with_name("message")
                        .help("tag message")
                        .takes_value(true)
                        .short("m")
                        .long("message"),
                ),
        );
    let matches = app.get_matches();
    if let Some(opt) = matches.subcommand_matches("all") {
        cmd_all(&opt)
    } else if let Some(opt) = matches.subcommand_matches("current") {
        cmd_current(&opt)
    } else if let Some(opt) = matches.subcommand_matches("next") {
        cmd_next(&opt)
    } else if let Some(opt) = matches.subcommand_matches("tag-next") {
        cmd_tag_next(&opt)
    } else {
        println!("{}", matches.usage());
        Ok(())
    }
}

# cs50-2023-rust
My wife is taking Harvard's cs50 course to learn programming. In order to encourage her and to also practice my rust, I'm going to follow along and perform each week's challenge but in Rust.

## Table of Contents
* [Links to Weeks](#links-to-weeks) - Links to the README.md files for each of the weeks' challenges.
* [Setting up this repo](#setting-up-this-repo) - How I set up this repo and thoughts on why.
* [Included Crates](#included-crates) - Crates I used in the challenges.
* [Feedback](#feedback) - Want to tell me a better way to do things?

# Links to Weeks
* [Week 0](https://github.com/Eragon615/cs50-2023-rust/blob/main/week0/README.md)
* Week 1 - [population](https://github.com/Eragon615/cs50-2023-rust/blob/main/week1/population/README.md) [mario](https://github.com/Eragon615/cs50-2023-rust/blob/main/week1/mario/README.md) [cash](https://github.com/Eragon615/cs50-2023-rust/blob/main/week1/cash/README.md)
* [Week 2]()
* [Week 3]()
* [Week 4]()
* [Week 5]()

# Setting up this repo
I wanted to have all the weeks in the same repo, and to name the files similarly to cs50 so I avoided using `cargo new`.  
But that turned out to be dumb. Using `cargo new --vcs=none name` turned out to be a much better way to avoid cargo trying to initialize git in an existing and superceding git repo. This way, VSCodium offered to link the Cargo.toml to my workspace settings in .vscode so that rust-analyzer would work automatically. Much better.

# Feedback
I'm still very new to programming. I work in IT, and sometimes I'll script something or write a little utility, but coding is not my day job. Therefore I have a decent understanding of core programming concepts, but very little real experience. I'm sure I'm making some poor choices as I write these. So feel free to open up an issue or PR to let me know of better ways, just be sure to explain why in small words. 

# Included Crates
These are the crates I used to complete challenges in some of the weeks' challenges.

| Crate Name | Version | Week(s) Used | Description |
| --- | --- | --- | --- |
| cs50 | 1.0.1 | 1 | A rust implementation of cs50.h from the cs50 class. |
![rusty-skywalker](./static/images/brand.png)

> A Powerful Bookmark Search Tool Powering Firefox & Chrome!

<p align="center">
    <img src="static/images/skywalker.png" height="150"><br>
    <a href="https://github.com/imskr/Rusty-Skywalker/releases"><img alt="GitHub release (latest by date including pre-releases)" src="https://img.shields.io/github/v/release/imskr/Rusty-Skywalker?include_prereleases&style=flat-square"></a>
    <a href="https://travis-ci.com/github/imskr/Rusty-Skywalker"><img alt="Travis (.com) branch" src="https://img.shields.io/travis/com/imskr/Rusty-Skywalker/master?style=flat-square"></a>
    <a href="https://github.com/imskr/Rusty-Skywalker/issues"><img alt="Github Issues" src="https://img.shields.io/github/issues/imskr/Rusty-Skywalker?color=orange&style=flat-square"></a>
    <a href="https://rustyskywalker.herokuapp.com"><img alt="Heroku Builds" src="https://pyheroku-badge.herokuapp.com/?app=rustyskywalker&style=flat-square"></a>
</p>
<hr noshade>

## Prerequisites

* Make sure you have Rust installed. 

```shell
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
* Rocket uses the nightly version of Rust so make sure you use that.

```sh
# from the root of the project
$ rustup override set nightly
```

## Installation

### Local
* Clone the repository.
```sh
$ git clone https://github.com/imskr/Rusty-Skywalker.git
```

* Build the project.
```sh
$ cargo build
```

* Run the project.
```sh
$ cargo run
```

* Visit localhost.
```
http://localhost:8000
```

## Usage

To test out a command, type in http://localhost:8000/search?cmd= followed by the command.

The following commands are supported by `Rusty-Skywalker`:
- "tw" -> redirects to twitter.com
- "tw @username" -> redirects to twitter.com/username

> Everything else redirects to a google search with your query.

## Queries and Commands

<p align="center">
    <img src="static/images/cmd.png">
</p>

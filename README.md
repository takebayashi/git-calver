# git-calver

`git calver` is a command line utility to manipulate git tags as [CalVer](https://calver.org).

## Demo

[![asciicast](https://asciinema.org/a/306537.svg)](https://asciinema.org/a/306537)

## Features

* List all versions: `git calver all`
* Show the latest version: `git calver current`
* Show the next version: `git calver next`
* Create a new tag with the next version: `git tag-next -m "New Release"`

## Installation

### Using brew

```
brew install takebayashi/git-calver/git-calver
```

### Using cargo

```
cargo install git-calver
```

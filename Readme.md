# GBC or git branch creator

CLI to help to create git branches out of your issue tracking.

## Install:

Use cargo to install

```shell
cargo install
```

Load the `cargo bin path` into your shell `bash/zsh/...`

```shell
export PATH=$HOME/.cargo/bin:$PATH
```

If you want to avoid load the `cargo bin path` every time just load the path into your shell.
[example here](https://ostechnix.com/how-to-add-a-directory-to-path-in-linux/)


## Usage:

You can set a global configuration:

```shell
gbc global-config
```

You can create a local configuration file named by **default** `gbc.json`: 

```shell
gbc init
```

You can run it and it will display a Fuzzy-find issues by name

```shell
gbc new feature
```

## Motivation:

I wanted to learn rust.

There are different kinds of harvester, some of them likes to garden flowers and 
admire how perfect and beauty they are, and others decide to garden fruits and 
eat them later.

## TBD:

* [ ] Jira issues driver 
* [ ] Github issues driver 
* [ ] Gitlab issues driver
* [ ] Notion driver

## Credits

- [fuzzy_finder](https://github.com/jamescoleuk/fuzzy_finder) for finding 
profile between multiple aws profiles.

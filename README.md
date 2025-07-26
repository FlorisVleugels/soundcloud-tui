# Soundcloud TUI

A Soundcloud client for the terminal written in Rust, inspired by spotify-tui.

![](https://tokei.rs/b1/github/FlorisVleugels/soundcloud-tui?category=code)

- [Soundcloud TUI](#soundcloud-tui)
  - [Installation](#installation)
  - [Connecting to Soundcloud’s API](#connecting-to-soundclouds-api)

## Installation

To be added

## Connecting to Soundcloud’s API (WIP)

`soundcloud-tui` needs to connect to Soundcloud’s API in order to find and stream music etc. 
API access requires you to sign up your own app

Instructions on how to set this up are below:

1. Go to the [Soundcloud help center](https://help.soundcloud.com/hc/en-us/requests/new)
1. Ask for API access (by registering an app) using "SoundCloud's Support Robot"
    - When your request is fulfilled you will be able to see your `Client ID` and `Client Secret` in your [apps](https://soundcloud.com/you/apps) page
1. Add `http://localhost:3000` to the Redirect URIs and click Save
1. You are now ready to authenticate with Soundcloud!
1. Go back to the terminal
1. Enter your `Client ID` and `Client Secret` in `~/.config/soundcloud-tui/client.yml`
1. Run soundcloud-tui
1. You will be redirected to an official Soundcloud webpage to ask you for permissions.
1. After accepting the permissions, you'll be redirected to localhost. If all goes well, the redirect URL will be parsed automatically.

And now you are ready to use the `soundcloud-tui` 🎉

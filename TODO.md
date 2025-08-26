# TODO

## General
- Change clientconfig and is_complete() cus client code is single use
- Add generic func for deserialize yaml
- Add anyhow errorhandling throughout, will improve the nesting of if lets cus can use ? and return Anyhow
- Clean up auth module
- Implement how to call the client api calls in a better way, and improve clientconfig
- Improve state management in app struct
- Add handlers for calling the client api methods from key presses
- Improve event handler code
- Add error page for main panel when couldnt load something


## Code
- Apply nested layout code instead of what i have now
- Deal with duplicate code in index increase /decrease
- Deal with duplicate code in ui draw functions (and tracks table)
- Deal with clone() calls, should be able to remove
- Move types and implementations for API responses out of api file
- Make current track / playback implementation better, can maybe merge them


## UI
- Add scrolling/ paging through the boxes
- Add highlight color to which pane is focused
- Add configurable color themes
- Add tracks body title depending on the playlist

## Audio
- Get audio streaming working with Symphonia and Rodio
    1. make get request and stream the bytes
    2. mp3 decode the stream
    3. append the stream to the rodio sink
- Improve error handling in playback.rs, shouldnt crash the app if something happens

## Bugs?
- Some fetch tracks doesnt work prolly cus json decode not working sometimes

# TODO

## General
- Change clientconfig and is_complete() cus client code is single use
- Add generic func for deserialize yaml
- Add anyhow errorhandling throughout
- Clean up auth module
- Implement how to call the client api calls in a better way, and improve clientconfig
- Improve state management in app struct
- Add handlers for calling the client api methods from key presses
- Improve event handler code


## Code
- Deal with duplicate code in index increase /decrease
- Deal with duplicate code in ui draw functions (and tracks table)
- Deal with clone() calls, should be able to remove
- Move types and implementations for API responses out of api file

## UI
- Add scrolling/ paging through the boxes
- Add highlight color to which pane is focused
- Add configurable color themes
- Add tracks body title depending on the playlist

## Bugs?
- Some fetch tracks doesnt work prolly cus json decode not working sometimes

# TODO

## General
- Improve draw loop polling (only while playing refresh every second)
- Change clientconfig and is_complete() cus client code is single use
- Add generic func for deserialize yaml
- Add anyhow errorhandling throughout, will improve the nesting of if lets cus can use ? and return Anyhow
- Clean up auth module
- Implement how to call the client api calls in a better way, and improve clientconfig
- Improve state management in app struct
- Add error page for main panel when couldnt load something
- Improve search functionality to cover other query params

## Performance
- Send multiple chunks at once to reduce cycles
- Separate draw loop from io (not an issue yet, might never be)
- Optimize parse_m3u8()

## Code
- Cleanup Status Bar code
- Apply nested layout code instead of what i have now
- Deal with duplicate code in index increase /decrease
- Deal with duplicate code in ui draw functions (and tracks table)
- Deal with clone() calls, should be able to remove
- Make current track / playback implementation better, can maybe merge them
- Fix library event handler, use Enum for library states

## UI
- Add scrolling/ paging through the boxes
- Add highlight color to which pane is focused
- Add configurable color themes
- Fix body title depending on if its a playlist or library item

## Audio
- Dont show warning messages from Rodio over tui
- Improve error handling in playback.rs, shouldnt crash the app if something happens

## Bugs?
- Some fetch tracks doesnt work prolly cus json decode not working sometimes

# simple-json-tui

A simple quality of life TUI application to make writing JSON less of a headache.

# TODO

**Priority**

- [ ] Implement nested objects
- [ ] Implement the Null JSON type

---

## Start page

- [ ] Implement a multiline block-drawn title
- [x] Implement a popup to write a filename or path for output JSON
- [ ] Maybe implement the ability to edit existing JSON file?

## Bottom status bar

- [ ] Implement a popup displaying all keybinds when `?` is clicked

## Config

- [ ] Implement a JSON format config file
- [ ] Create a JSON Schema for users to use
- [ ] Should be found in user's `~/.config/simple-json` directory
- [ ] Parse with `serde_json` into a predefined `struct` to allow for easy code implementation if file is found

## CLI

- [ ] Implement `clap` for CLI argument/option parsing

## Key/Values

- [ ] Implement easy snake_case or camelCase for input

## Preview

- [ ] Figure out word/text wrapping
- [ ] Figure out how to edit created fields

## New Features

- [ ] Implement JSON schema (pretty much developing a strictly JSON tui-IDE at this point)
- Need to make http requests to the specified Schema URI
- Need to parse Schema URI to a struct? to implement autocomplete fields (basically a Language Server)
- Need to validate user JSON against Schema

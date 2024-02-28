# Orbit Roadmap

## 0.1.0
- [ ] terminal cell
- [ ] cell buffer
- [ ] render loop
    - [ ] immediate rendering
- [ ] widgets
    - [ ] span
    - [ ] text
    - [ ] box
- [ ] rendering
    - [ ] viewport
    - [ ] render frame
    - [ ] widget tree

## Features
- [ ] widget properties
    - [ ] modifiers
        - fg, bg, italic, etc
    - [ ] state
    - [ ] composition
    - [ ] conditional rendering
    - [ ] children widgets
- [ ] builtin widgets
    - [ ] span
        - a contiguous set of styled text
    - [ ] line
        - a single line of spans
    - [ ] text
        - a collection of contiguous lines, line wrapped
    - [ ] block/box
        - a wrapper widget to display borders
        - [ ] horizontal
        - [ ] vertical
- [ ] rendering
    - [ ] viewport
        - [ ] fullscreen
            - alternate buffer with full terminal width/height
        - [ ] inline
            - set height, variable width
        - [ ] fixed area
            - set height/width
    - [ ] render diffing
        - check what's been changed since last frame and only update the changes
    - [ ] buffers
        - a collection of mutable terminal cells
    - [ ] cell
        - a single character slot

## API
- [ ] render loop
    - [ ] render event conditions
        - if a specified event occurs, update the widget/terminal frame
- [ ] terminal rendering
    - [ ] lazy rendering
    - [ ] immediate rendering
- [ ] widget tree
    - the collection of all the widgets in a frame (basically a glorified layout tree)
    - widget
        - re-rendering is determined by a condition --> toggles the re-rendering flag
- [ ] conditions
    - events (or state updates)

## Long-term Goals

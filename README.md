# Diamond Rust

## Authors
- Krystyna Gasińska (@monty930 on GitHub)

## Description
The idea is to create a game similar to „Diamond rush” (the game that used to be on old cellphones). It would have one level, diamonds to collect (points), enemies to avoid (without combat system), lives, keys to collect, doors opened by keys, bushes to destroy and falling stones. 
Gameplay of original game: https://www.youtube.com/watch?v=W5jp-VyLEHY
Unlike the original game there won’t be fire, hammer (to destroy brittle obstacles) or chests. The stones will fall only if there is nothing below them.

## Features
- scores
- enemies
- game state saving and loading
- falling stones
- system of keys and doors

## Plan
In the first part I'm going to implement the map with diamonds, bushes, keys and doors.

In the second part I'm going to add enemies, lives, stones and saving-loading system.

## Libraries
- Bevy
- Serde

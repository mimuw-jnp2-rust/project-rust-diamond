# Diamond Rust

## Authors
- Krystyna Gasińska (@monty930 on GitHub)

## Description
The idea is to create a game similar to „Diamond rush” (the game that used to be on old cellphones). It would have one level, diamonds to collect (points), enemies to avoid (without combat system), lives, keys to collect, doors opened by keys, bushes to destroy falling stones and a hammer.
Gameplay of original game: https://www.youtube.com/watch?v=W5jp-VyLEHY

## Features
- scores
- enemies
- game state saving and loading
- falling stones
- system of keys and doors
- riddles with a goal to defeat enemies

## Plan
In the first part I'm going to implement the map with diamonds, bushes, keys and doors.

In the second part I'm going to add enemies, lives, stones, saving-loading system and a hammer (to destroy brittle obstacles and immobilize enemies). The stone will fall if there is nothing below it and it will roll to the left or right if there is a stone below it. Falling stone can also kill the enemy or the player.
I will also add riddles like the one at 07:54 in linked gameplay: https://www.youtube.com/watch?v=W5jp-VyLEHY

## Libraries
- Bevy
- Serde

## Part one progress
- Player movement. Player is moving to adjacent "grass" tiles. The non-continous movement (moving from one tile directly to the next one) is intentional.
- Map creation. The map is created before the game and consists of wall (obstacles) and grass. The arrangement of tiles is determined by the text file (/assets/map.txt). The player can move only on "grass" tiles.
- TODO: Bushes.
- TODO: Keys.
- TODO: Doors.

## Credits
Player assets thanks to: https://edermunizz.itch.io/pixel-art-rpg-character-creator

Part one was created with assistance of following sources:

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

## Credits
Player assets thanks to: https://edermunizz.itch.io/pixel-art-rpg-character-creator

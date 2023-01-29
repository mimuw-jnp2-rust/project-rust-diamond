# Diamond Rust

## Part one progress (All planned features implemented)
- Player movement. Player is moving to adjacent "grass" tiles. The non-continous movement (moving from one tile directly to the next one) is intentional.
- Map creation. The map is created before the game and consists of wall (obstacles) and grass. The arrangement of tiles is determined by the text file (/assets/map.txt). The player can move only on "grass" tiles.
- Bushes. Bushes occur on the map. They disapear when being stepped by the player.
- Keys. There are keys to be found on the map. Player can collect them (they disapear from the map and are added to the player inventory).
- Doors. Player can open door (it disappears from the map) in exchange for one of the keys from the equipment. With no keys player cannot go through the door.
- Diamonds. Appear on the map. Player can collect them; they are added to the equipment.
- Brittle obstacles. Only appear on the map (purple wall).
- Inspector-egui debugging system. Allows to see components and data of entities on the screen. May be removed later. 

## Part two progress
- Enemies. Enemies move up and down or left and right (from wall to wall).
- Lives. Player has lives that can be collected from the map. Lives are lost after collision with an enemy.
- Animation. The player turns in the direction of movement. Steps are animated. After collision with an enemy (after losing a life) player turns red for a few seconds. During this time player cannot be hit again. After losing last life player is defeated and is not moving anymore.
## Not implemented: Stones, saving loading system and hammer.

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

## Credits
Player assets thanks to: https://edermunizz.itch.io/pixel-art-rpg-character-creator

Other tiles assets thanks to: https://opengameart.org/

Part one was created with assistance of following tutorials:

https://www.youtube.com/watch?v=Yb3vInxzKGE

https://www.youtube.com/watch?v=WnUzWuaMzuM

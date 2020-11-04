# Sun Prison

Sun Prison (working title) is a WIP turn-based meditation on Rubik's cube, [Sokoban](https://github.com/ropewalker/bevy_sokoban/), and roguelikes, being done with [Bevy engine](https://bevyengine.org/) (and no previous experience in gamedev). 

![demo gif](/assets/screenshots/demo.gif)

The game is in the *very* early stages of development and is being implemented by a single person ([me](https://github.com/ropewalker)), so not only it is barely playable, but also commit history is a mess and backlog consists mostly of random marginal notes. On the bright side, it is already possible to get lost in the dark or to be eaten by zombies.

## How to play

### Current goal and rules of the game

The goal (currently, likely to be changed) is to find and step on the exit tile. You can push boxes but not walls and you can rotate layers of the cube at your will. You have three health points and lose one every time a monster tries to step on the tile you are standing one, which may lead to your death and losing the game.

There are three types of monsters currently, the only difference between them is how they track you: 
- Green monsters (called “zombies” in code) have conic field of view and can lose your tracks easily;
- Purple monsters (called “ghouls” in code) see around them, just like the player character does;
- Red monster (called “demon” in code) always knows where you are. There’s no hiding from the red monster. 

The game is turn-based, your turn ends every time you perfrom (or try to perform) any movement by pressing W, S, or SPACE, or when you explicitly end your turn by pressing E. Next turn starts when all enemies make their movements. Highlighting current layer by pressing TAB or turning around by pressing S or D don't pass the turn.

### UI

- Game map represents six sides of a cube which you can explore (think The Little Prince’s asteroid, but cubic). Layers of the cube can be independently rotated (think Rubik's cube).
- Black-colored character represents you, other characters represent monsters that are trying to eat you. 
- White boxes can be moved, like in [Sokoban](https://github.com/ropewalker/bevy_sokoban/); striped boxes represent walls and cannot be moved.
- Tile with blue frame represents exit.
- Some information is currently printed in console.
- The game doesn't (yet) restart automatically when you win or lose, sorry about that.

### Controls

Use WASD to move and turn around and SPACE to rotate the layer you are standing on in the direction you are looking in. TAB will highlight the layer that will be rotated by pressing SPACE. Pressing E ends the turn.

## Future plans (besides obvious bug fixing and performance updates)

- [ ] UI (or at least some text on the screen);
- [ ] Random map generation;
- [ ] New interactable objects and enemies;
- [ ] Day/night cycle (maybe);
- [ ] Combat;
- [ ] Slightly more interesting win condition;
- [ ] Better art;
- [ ] 3D (maybe, at some point).

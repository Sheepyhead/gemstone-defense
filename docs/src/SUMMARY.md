# Overview

## Big Picture

### Elevator Pitch

Dig for gems and use their mystical powers to destroy hordes of gem thieves!

### Target Audience

People with a fondness for simple tower defense games with a surprisingly elaborate end game, people with nostalgia for games like Warcraft 3.

### Themes

Simple stuff, fighting invaders and so on. Nothing deep.

## Mechanics

### Moment-to-moment Loop

The game consists of two phases: Build, and defend.
- Build: The player digs five 2x2 holes in occupied spaces on the board, once all five holes are dug, reveal a random assortment of gem towers. The player either picks one of the individual towers to keep, or picks a special tower recipe from among the five gems, if present. The other four gems turn into dirt piles that don't attack but do block creeps, and the defend phase starts.
- Defend: Creeps spawn at the starting zone and make their way through a number of designated points on the map, moving around the placed towers while the towers attack the creeps. Any creep that passes through the map to the end causes the player to lose a life.
### Game Loop

The player starts with an empty map, digging five holes and getting only a single tower for the first round of defense. Then the player uses the dirt piles and picked gem towers to maze the creeps, getting an additional tower each round. In the defend phase you can also combine towers you have that satisfy a special tower recipe, leaving a dirt pile on the other used towers.

### Progression Loop

The player develops a better understanding for tower powers, special tower recipes, and mazing strategies. Maybe special towers and their recipes are only revealed once the ingredients have been present for a game.

### Objects

Creeps: The monsters that run through the maze the player has to kill.
Special towers: Towers made by combining other towers. Some can be upgraded with gold.
Towers/gems: The main offensive unit of the game, blocks creep paths and attacks creeps and/or buffs/debuffs.

### Actions

Dig/build: In the Build phase, creates an open hole. Dig five times and the gems are revealed.

### Resources

Gold: Killing creeps yields gold which is used to upgrade gem chance and special towers

### Design Invariants

The game will be fully 2d with no map movement or zoom.

### Design Constraints

What problems will be deal-breakers for players? What restrictions are we imposing on ourselves?

### Design Tolerances

What unusual flaws or unconventional choices will our players accept?

### Systems and Hooks

How can we generate interesting complexity?

## Technical

### Technical Strategy

Initial support only for Linux and maybe Windows. Ideally support all possible platforms down the line.

### Technical Constraints

What unexpected constraints are we likely to encounter?

### Technical Tolerances

What parts will be easier than you might expect? Where can we cut corners?

## Art

### Art Strategy

What basic style and techniques are we going to use? How big is our budget?

### Art Constraints

What unexpected constraints are we likely to encounter?

### Art Tolerances

What parts will be easier than you might expect? Where can we cut corners?

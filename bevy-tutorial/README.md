# Bevy Tutorial

Trying to learn `game dev` using a code first approach. I chose the [bevy engine](https://bevyengine.org/) because I am
looking forward to learn rust and [ECS](https://en.wikipedia.org/wiki/Entity_component_system).

Extensive docs can be found [here](https://bevyengine.org/learn/quick-start/introduction/) and
[here](https://bevy-cheatbook.github.io).

Examples can be found [cloning the repo](https://github.com/bevyengine/bevy), or
[here](https://bevyengine.org/examples/).

For deep dive in ECS I want to eventually checkout [this engine](https://github.com/tjdevries/ocaml-engine), mostly
because it has dev logs.

## Projects

- [Tutorial](./tutorial)

## Notes

- On a real project, make sure to follow the performance optimizations
  [listed on the setup guide](https://bevyengine.org/learn/quick-start/getting-started/setup/).

## Issues

I am using `POP_OS!` because I am a basic bitch, so GNOME and the X11 stuff are very old, and I am experiencing screen
freezes (only on the monitor running the app) even when running the `bevy` examples (they don't seem to happen if I
explicitly run them with `--features X11`). Following the tutorial and some issues I found no the repo, I tried doing
these steps to fix the issue:

- install [Vulkan SDK](https://vulkan-tutorial.com/Development_environment#page_Linux)
- enable the X11 feature explicitly

After all, nothing seems to make it stable, but the issue might be happening only when I resize the window.

My first contact with the engine, following the
[Tutorial](https://www.youtube.com/watch?v=TQt-v_bFdao&list=PLVnntJRoP85JHGX7rGDu6LaF3fmDDbqyd).

With adaptations for version `0.15`, using also [this video](https://www.youtube.com/watch?v=yFOPtYwnDjU).

This [migration guide](https://bevyengine.org/learn/migration-guides/0-14-to-0-15/) is also life saver and the example
on [the bevy repo](https://github.com/bevyengine/bevy/blob/main/examples/2d/sprite_animation.rs).

TODO: 

  see https://gist.github.com/paul-hansen/8c87fe5cb3bab1850943036c7af66eb3
  see https://stackoverflow.com/questions/76428101/resizing-windows-in-bevy


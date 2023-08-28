# Deprecated

This game has been moved to [the Ambient repository](https://github.com/AmbientRun/Ambient/tree/main/guest/rust/packages/games/afps).

---

# AFPS Mods

AFPS is a moddable FPS game written with the Ambient engine.

## Run the game

You need to install Ambient first to run it.

## Mod

Different from most Ambient projects, this FPS game is organised with different mods.

The `Cargo.toml` includes several `bins` in orders. You can try to uncomment them group by group and run `ambient run --clean-build` to see how these mods are put up together.

> If it's your first time to run, just call `ambient run` without `--clean-build`.

```toml

# Group 1: show the UI
# This include the aiming cross in the middle of the screen
# Use TAB key to toggle the scoreboard
# The "Rule" mod provides the calculation for player's score/health, etc.
# It's also the foundation of the future animation mod
# When the player health changes, "Hit" or "Death" animation will be triggered

[[bin]]
name = "fpsui_client"
path = "src/fpsui/client.rs"
required-features = ["client"]

[[bin]]
name = "fpsui_server"
path = "src/fpsui/server.rs"
required-features = ["server"]

[[bin]]
name = "fpsrule_server"
path = "src/fpsrule/server.rs"
required-features = ["server"]

# Group 2: show the scene
# For now, I only put a demo quad there, you can edit this as the game scene

[[bin]]
name = "scene_server"
path = "src/scene/server.rs"
required-features = ["server"]

# Group 3: show the players' model
# This will show a T-pose player and you cannot move

[[bin]]
name = "fpsmodel_client"
path = "src/fpsmodel/client.rs"
required-features = ["client"]

[[bin]]
name = "fpsmodel_server"
path = "src/fpsmodel/server.rs"
required-features = ["server"]

# Group 4: add the movement system
# Now you can move your model with mouse/keyboard

[[bin]]
name = "fpsmovement_client"
path = "src/fpsmovement/client.rs"
required-features = ["client"]

[[bin]]
name = "fpsmovement_server"
path = "src/fpsmovement/server.rs"
required-features = ["server"]

# Group 5: Animation

[[bin]]
name = "fpsanim_server"
path = "src/fpsanim/server.rs"
required-features = ["server"]

# Group 6: The audio system

[[bin]]
name = "fpsaudio_client"
path = "src/fpsaudio/client.rs"
required-features = ["client"]

[[bin]]
name = "fpsaudio_server"
path = "src/fpsaudio/server.rs"
required-features = ["server"]

# Group 7: Optional and unfinished Zombie system
# The Zombie rules and health/hit anim is not included yet
# They simply spawan and chase the nearest player

# [[bin]]
# name = "zombie_server"
# path = "src/zombie/server.rs"
# required-features = ["server"]

# [[bin]]
# name = "zombie_client"
# path = "src/zombie/client.rs"
# required-features = ["client"]

```

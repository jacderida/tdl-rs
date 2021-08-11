# Terminal Doom Launcher

Terminal Doom Launcher, or TDL, is a launcher application for Doom. A launcher is intended to make it easier to manage the multitude of different Source Ports, MegaWADs, supplementary WADs, demos, and also keep track of certain things, like favourite maps.

As the name implies, rather than being driven by a GUI, TDL runs from the terminal. It has both a CLI and an interactive interface. The intention is for it to be fast to use for people who are already very familiar with a terminal environment.

I do development on Linux, but I play on Windows, so I intend to have the application be fully functional in Powershell.

## Using TDL

- [Installation](#installation)
    - [Windows](#windows)
    - [Linux](#linux)
- [Source Ports](#source-ports)
- [Profiles](#profiles)

## Installation

### Windows

There will be a Chocolatey package and Powershell script available for installation on Windows.

### Linux

There will be a Bash script available for installation on Linux.

## Source Ports

At least one source port must be added, as pretty much every other command will make a reference to a source port.

A previously downloaded source port can be added using the `source-port add` command:
```
tdl source-port add "prboomumapinfo" "~/doom/source-ports/prboom-2.6um/prboom-plus" "2.6um"
```

The first argument must be one of the supported source port types. Run `tdl source-port add --help` to see a list of valid values.

## Profiles

Profiles provide a way to play the game using different options and configurations. You need at least one profile to function as the default. The recommended use for the default profile is the way you must commonly like to play the game. So it would use your favourite source port and the common options you typically play with. For example, I mostly like to play while listening to my own music or a podcast, so my default profile will run the game with no music, using the DSDA source port. I may have a different profile for recording a demo or generating a video from a demo. I might have different profiles for experimenting with different source ports, and so on.

When you run the `tdl play` command, it will launch with the default profile. If you want to use a different profile, you can use the optional `--profile` argument.

Create a profile using the `profile add` command. To see all the possible options, use `tdl profile add --help`. Most of the options correspond to the arguments the game accepts.

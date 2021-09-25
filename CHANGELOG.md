# Changelog

## Version 0.1.0

This release implements commands that give basic, usable functionality to the launcher. All the settings and state for the launcher are maintained just using simple, serialized json, saved on the local file system. There is no database, and I'd like to keep it that way.

* The `source-port add` command: this saves the specified source port to the launcher's settings. This will eventually be largely unnecessary and made obsolete by an `install` command, which will download and install the source port. However, the command will still be necessary for source ports that don't host their binaries on Github releases.
* The `profile add` command: creates a new profile, which specifies a way in which the user would like the game to run.
* The `wad lsdir` command: lists all the directory entries in a WAD. This probably won't be too useful for users, but it was a good way to learn how to parse a WAD file.
* The `iwad import` command: imports an IWAD into the user's collection. Information like map names are statically encoded in the TDL binary, since for the original IWADs, this information was embedded inside graphics data.
* The `play` command: with a source port, profile and IWAD imported, this command can launch the source port at a specific map. The only port that's been worked with so far is PrBoom. This will obviously be extended in due course. If you don't specify a map on the command line, TDL will use a fuzzy finder to allow the user to select a map. This is provided via the [skim](https://github.com/lotabout/skim) library on Linux and the `fzf` program on Windows, since skim doesn't yet support Windows.

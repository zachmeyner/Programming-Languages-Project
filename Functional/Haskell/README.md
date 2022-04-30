# Haskell
Haskell is a functional language that from what I can tell is mostly used for academic purposes.

What I have made takes advantage of cabal to build the project.

To get the language I installed GHCup from the haskell website with 
```bash
curl --proto '=https' --tlsv1.2 -sSf https://get-ghcup.haskell.org | sh
```
and made sure to get HLS, Stack, and cabal.

## Build
To build you go into the `tictactoe` directory and run
```bash
cabal build :all
```
and to run the program you run
```bash
cabal run :all
```
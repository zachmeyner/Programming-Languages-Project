# C++
I don't need to write instructions for this do I?

## Cmake
To build I will be using CMake to auto-generate the Makefile for the program.
I am using the latest debian release of CMake to generate the Makefile for this project. To install it simply run
```bash
sudo apt install cmake
```
To build it go to the `animals` directory and type into a terminal
```bash
cmake -S . -B build/
```
This will create a makefile in `animals/build/`

From here change directories to `build` and type `make animals` into a terminal. This will generate an `animals` file. To run it type `./animals` into a terminal.
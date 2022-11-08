#!/bin/bash

find ares/test -iname *.h -o -iname *.cpp -o -iname *.hpp | xargs clang-format -i &&
cd desktop-ui &&
make local=false &&
out/ares.app/Contents/MacOS/ares "/Users/mikhailhogrefe/Documents/Games/Game Boy/Tetris.gb"

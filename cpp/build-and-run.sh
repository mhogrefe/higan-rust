find higan/test -iname *.h -o -iname *.cpp -o -iname *.hpp | xargs clang-format -i &&
make -C higan &&
make -C icarus &&
make -C higan install &&
make -C icarus install &&
./higan/out/higan.app/Contents/MacOS/higan


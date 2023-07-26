cd ./corelib; cargo build; cd ..;
g++ test/main.cpp -L ./corelib/target/debug/ -I ./UniversalUI/include/ -o test/main -lcorelib -Wl,-rpath ./corelib/target/debug/;
./test/main

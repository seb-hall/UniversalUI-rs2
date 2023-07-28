echo "Building UniversalUI_Native...";
cd ./UniversalUI_Native; cargo build; cd ..;
echo "Done!";
echo "Building test application";
g++ test/main.cpp -L ./UniversalUI_Native/target/debug/ -I ./UniversalUI/include/ -o test/main -lUniversalUI_Native -lX11 -lXext -Wl,-rpath ./UniversalUI_Native/target/debug/;
echo "Done!";
echo "Running test application";
./test/main

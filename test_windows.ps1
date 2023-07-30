echo "Building UniversalUI_Native...";
cd ./UniversalUI_Native; cargo build; cd ..;
echo "Done!";
echo "Building UniversalUI_Graphics...";
cd ./UniversalUI_Graphics; cargo build; cd ..;
echo "Done!";
echo "Building test application";
g++ test/main.cpp -L ./UniversalUI_Native/target/debug/ -L ./UniversalUI_Graphics/target/debug/ -I ./UniversalUI/include/ -o test/main -lUniversalUI_Native -lUniversalUI_Graphics -Wall -g -mconsole;
cp ./UniversalUI_Native/target/debug/UniversalUI_Native.dll ./test/
cp ./UniversalUI_Graphics/target/debug/UniversalUI_Graphics.dll ./test/
echo "Done!";
echo "Running test application";
./test/main

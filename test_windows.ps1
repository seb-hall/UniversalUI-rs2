echo "Building UniversalUICore...";
cd ./UniversalUICore; cargo build; cd ..;
echo "Done!";
echo "Building test application";
g++ test/main.cpp -L ./UniversalUICore/target/debug/ -I ./UniversalUI/include/ -o test/main -lUniversalUICore -Wall -g -mconsole;
cp ./UniversalUICore/target/debug/UniversalUICore.dll ./test/
echo "Done!";
echo "Running test application";
./test/main

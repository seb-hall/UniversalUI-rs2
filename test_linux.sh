echo "Building UniversalUICore...";
cd ./UniversalUICore; cargo build; cd ..;
echo "Done!";
echo "Building test application";
g++ test/main.cpp -L ./UniversalUICore/target/debug/ -I ./UniversalUI/include/ -o test/main -lUniversalUICore -Wl,-rpath ./UniversalUICore/target/debug/;
echo "Done!";
echo "Running test application";
./test/main

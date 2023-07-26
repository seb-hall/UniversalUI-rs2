#include <iostream>

extern "C" {

    bool run();

    void run_call_c() {
        std::cout << "Hi from c\n";
    }
}
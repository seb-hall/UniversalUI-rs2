#include <stdio.h>

extern "C" {
    void hello_from_rust();
}

int main() {
    printf("Hello from C++!\n");
    hello_from_rust();
}
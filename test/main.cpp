#include "core/lib.h"
#include "core/debug.h"

int main() {

    debug_info("Starting test application!");

    if (core_init()) {
        debug_info("Initialised OK!");
        return 0;
    } else {
        debug_critical("Initialisation Error!");
        return -1;
    }

}
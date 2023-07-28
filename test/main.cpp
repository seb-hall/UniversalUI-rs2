#include "core/native/lib.h"
#include "core/native/window.h"
#include "core/native/event.h"
#include "core/base/general.h"
#include "core/base/geometry.h"

#include <stdio.h>

// Function to handle window events, called from your C++ code
void handleWindowEvent(uID windowId, int eventType) {
    // Call the Rust function passing the window event information
    printf("I got called back!\n");
}

int main() {

    printf("starting test...\n");

    if (!native_init()) {
        return -1;
    }

    uID window1 = create_window("window 1", uSize { 800.0, 600.0 });

    printf("got to here!\n");
    while (true) {
        printf("got to here!\n");
        get_events();
    }



    return 0;
}
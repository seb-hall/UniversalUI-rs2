#include "core/native/lib.h"
#include "core/native/window.h"
#include "core/native/event.h"
#include "core/base/general.h"
#include "core/base/geometry.h"

#include <stdio.h>

// Function to handle window events, called from your C++ code
void handleWindowEvent(uID windowId, int eventType, float parameters[4]) {
    // Call the Rust function passing the window event information
    switch (eventType) {
        case WINDOW_RESIZED: 
            printf("window resized: %.1f %.1f\n", parameters[0], parameters[1]);
            break;
        default: 
            printf("window close button pressed\n");
            destroy_window(windowId);
            break;

    }
}

int main() {

    printf("starting test application...\n");

    if (!native_init()) {
        return -1;
    }

    uID window1 = create_window("window 1", uSize { 800.0, 600.0 });
    uID window2 = create_window("window 2", uSize { 800.0, 600.0 });
    uID window3 = create_window("window 3", uSize { 800.0, 600.0 });

    registerWindowEventCallback(handleWindowEvent);

    while (true) {
       get_events();
    }

    return 0;
}
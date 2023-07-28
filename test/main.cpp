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
    uID window2 = create_window("window 2", uSize { 800.0, 600.0 });
    uID window3 = create_window("window 3", uSize { 800.0, 600.0 });

    registerWindowEventCallback(handleWindowEvent, nullptr);

    set_window_visibility(window1, false);
    

    return 0;
}
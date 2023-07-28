#include "core/native/lib.h"
#include "core/native/window.h"
#include "core/native/event.h"
#include "core/base/general.h"
#include "core/base/geometry.h"

#include <stdio.h>

int main() {

    printf("starting test...\n");

    if (!native_init()) {
        return -1;
    }

    uID window1 = create_window("window 1", uSize { 800.0, 600.0 });
    uID window2 = create_window("window 2", uSize { 800.0, 600.0 });
    uID window3 = create_window("window 3", uSize { 800.0, 600.0 });

    set_window_visibility(window1, false);
    //destroy_window(window1);

    while (true) {
        get_events();
    }

    return 0;
}
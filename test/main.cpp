#include "core/lib.h"
#include "core/base.h"
#include "core/debug.h"
#include "core/window.h"

#include <stdio.h>

int main() {

    debug_info("Starting test application!");

    uPoint myPoint;
        myPoint.x = 10.0;
        myPoint.y = 50.0;


        uPoint recievedPoint = check_point(myPoint);

        printf("I got x: %f y: %f\n", recievedPoint.x, recievedPoint.y);
    
    uWindowInfo myWindowInfo;
    myWindowInfo.title = "Hello window 1";
    myWindowInfo.size = { 800.0, 600.0 };

    uWindowHandle myWindowHandle = create_window(myWindowInfo);

    if (core_init()) {
        debug_info("Initialised OK!");
        return 0;
    } else {
        debug_critical("Initialisation Error!");
        return -1;
    }

}
#include "core/lib.h"
#include "core/base.h"
#include "core/debug.h"
#include "core/window.h"
#include "core/run.h"

#include <stdio.h>

int main() {

    debug_info("Starting test application!");

    core_init();
    
    uWindowInfo myWindowInfo;
    myWindowInfo.title = "Hello window 1";
    myWindowInfo.size = { 2560.0, 1440.0 };

    uWindowInfo myWindowInfo2;
    myWindowInfo2.title = "Hello window 2";
    myWindowInfo2.size = { 2560.0, 1440.0 };

    for (int i = 0; i < 250; i++ ) {
        create_window(myWindowInfo);
    }
    uWindowHandle myWindowHandle1 = create_window(myWindowInfo);

    uWindowHandle myWindowHandle2 = create_window(myWindowInfo2);

    printf("got to here ok");
    run();


}
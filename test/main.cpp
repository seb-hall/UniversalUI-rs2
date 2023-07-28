#include "core/native/lib.h"
#include "core/base/geometry.h"

#include <stdio.h>

int main() {

    printf("starting test...\n");

    if (!native_init()) {
        return -1;
    }

    return 0;
}
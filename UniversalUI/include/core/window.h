#include "base.h"

struct uWindowInfo {
    const char* title;
    uSize size;
};

struct uWindowHandle {
    void* raw_handle;
};

extern "C" {

    struct uWindowInfo;
    struct uWindowHandle;

    uWindowHandle create_window(uWindowInfo window_info);

}

#include <cstdint>

#include "base/geometry.h"

struct uWindowProvider {

};

struct uWindowInfo {
    const char* title;
    uSize size;
};

struct uWindowHandle {
    uint64_t handle;
};

extern "C" {

    //  define structs
    struct uWindowInfo;
    struct uWindowHandle;

    uWindowHandle create_window(uWindowInfo window_info);
    bool is_window_valid(uWindowHandle);

}
#include "../base/general.h"
#include "../base/geometry.h"

extern "C" {

    uID create_window(const char* title, uSize size);
    void destroy_window(uID id);
    bool get_window_visibility(uID id);
    void set_window_visibility(uID id, bool visibility);
    const char* get_window_title(uID id);
    void set_window_title(uID id, const char* title);
    uSize get_window_size(uID id);
    void set_window_size(uID id, uSize size);
    bool get_window_fullscreen(uID id);
    void set_window_fullscreen(uID id, bool fullscreen);

}
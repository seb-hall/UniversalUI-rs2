#include "../base/general.h"

extern "C" {

    //  get events
    void get_events();

    typedef void (*WindowEventCallback)(uID windowId, int eventType, float parameters[4]);

    void registerWindowEventCallback(WindowEventCallback callback);

}

#define UEVENT_NULL             0
#define WINDOW_CREATED          1
#define WINDOW_DESTROYED        2
#define WINDOW_MOVED            3
#define WINDOW_RESIZED          4
#define WINDOW_GAINED_FOCUS     5
#define WINDOW_LOST_FOCUS       6
#define WINDOW_NEEDS_REDRAW     7
#define WINDOW_CLOSE_PRESSED    8
#define CURSOR_MOVED            9
#define CURSOR_ENTERED          10
#define CURSOR_LEFT             11
#define BUTTON_DOWN             12
#define BUTTON_UP               13
#define KEY_DOWN                14
#define KEY_UP                  15

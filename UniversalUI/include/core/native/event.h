#include "../base/general.h"

extern "C" {

    //  get events
    void get_events();

    typedef void (*WindowEventCallback)(uID windowId, int eventType);

    void registerWindowEventCallback(WindowEventCallback callback, void* userData);

}
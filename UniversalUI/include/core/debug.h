
extern "C" {

    //  information only, no issues
    void debug_info(const char* input_string);
    
    //  warning, no significant issues to functionality but
    //  non ideal implementation
    void debug_warning(const char* input_string);
    
    //  error, wrong implementation and functionality affected.
    //  Typically used when using a function incorrectly.
    void debug_error(const char* input_string);
    
    //  critical error, app about to crash
    void debug_critical(const char* input_string);
    
}
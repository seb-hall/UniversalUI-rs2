# src/CMakeLists.txt

set(CMAKE_CXX_STANDARD 14)

set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTOUIC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_INCLUDE_CURRENT_DIR ON)

set(SOURCE main.cpp)
add_executable(main ${SOURCE})
get_target_property(UNIVERSALUI-NATIVE-RS_DIR UniversalUI-Native-rs LOCATION)
#target_link_libraries(main Qt5::Widgets)
target_link_libraries(main ${UNIVERSALUI-NATIVE-RS_DIR}/libUniversalUI_Native_rs.so)
add_dependencies(main UniversalUI-Native-rs)

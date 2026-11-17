cmake_minimum_required(VERSION 3.22.1)
project(minimalium_jni LANGUAGES C CXX)

add_library(minimalium_core SHARED IMPORTED)
set_target_properties(minimalium_core PROPERTIES
    IMPORTED_LOCATION "${CMAKE_CURRENT_SOURCE_DIR}/../jniLibs/${ANDROID_ABI}/libminimalium_core.so"
)

add_library(minimalium_jni SHARED minimalium_jni.cpp)
target_include_directories(minimalium_jni PRIVATE "${CMAKE_CURRENT_SOURCE_DIR}/../../../../include")
find_library(android-log-lib log)

target_link_libraries(minimalium_jni
    minimalium_core
    ${android-log-lib}
)

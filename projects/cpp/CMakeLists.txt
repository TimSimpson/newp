# *********************************************************************
# {{scream_case_name}}
#       {{description}}
# *********************************************************************
cmake_minimum_required(VERSION 3.9.0 FATAL_ERROR)

if(NOT DEFINED PROJECT_NAME)
    set(NOT_SUBPROJECT ON)
endif()

project({{kebab_case_name}} CXX)

include(CTest)
include(CMakePackageConfigHelpers)

# Allow user to ask explicitly to build tests
option({{scream_case_name}}_Build_Tests "Build tests when BUILD_TESTING is enabled."
       ${NOT_SUBPROJECT})

add_library(
    {{snake_case_name}}
    ${CMAKE_CURRENT_SOURCE_DIR}/include/{{name}}.hpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/{{name}}.cpp)
set_target_properties({{snake_case_name}} PROPERTIES OUTPUT_NAME "{{kebab_case_name}}")

# Mandate the use of at least C++17 by everything that uses this
target_compile_features({{snake_case_name}} PUBLIC cxx_std_17)

target_include_directories(
    {{snake_case_name}}
    PUBLIC $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
           $<INSTALL_INTERFACE:include>
    PRIVATE src)

# This is built as a "shared library" in sarcastic air quotes. It's only
# made that way to make linking faster, and relies on consumers using the same
# version of the runtime it was built with. IIRC this used to not be that big
# of a problem with VS, but now it is, thus the disabled warnings.
if(BUILD_SHARED_LIBS)
    target_compile_definitions(
        {{snake_case_name}}
        PUBLIC {{scream_case_name}}_API_DYNAMIC
        PRIVATE {{scream_case_name}}_API_CREATE)
    if(MSVC)
        target_compile_options({{snake_case_name}} PRIVATE /wd4251 /wd4275)
    endif()
endif()

# *********************************************************************
# Tests and Drivers / Demos
# *********************************************************************

if(BUILD_TESTING AND {{scream_case_name}}_Build_Tests)
    message(INFO " CMAKE_MODULE_PATH=${CMAKE_MODULE_PATH}")
    find_package(Catch2 REQUIRED)

    add_executable({{snake_case_name}}_cli WIN32
                   ${CMAKE_CURRENT_SOURCE_DIR}/demos/{{name}}_cli.cpp)
    target_link_libraries({{snake_case_name}}_cli {{snake_case_name}})

    function(make_test exe_target)
        if("${CMAKE_SYSTEM_NAME}" MATCHES "Emscripten")
            add_test(NAME "test_${exe_target}"
                     COMMAND node $<TARGET_FILE:${exe_target}>)
        else()
            add_test(NAME "test_${exe_target}" COMMAND ${exe_target})
        endif()
    endfunction()

    add_executable({{snake_case_name}}_test
                   ${CMAKE_CURRENT_SOURCE_DIR}/tests/{{name}}_test.cpp)
    target_link_libraries({{snake_case_name}}_test {{snake_case_name}}  Catch2::Catch2)
    make_test({{snake_case_name}}_test)

    if(BUILD_SHARED_LIBS)
        if(MSVC)
            target_compile_options({{snake_case_name}}_test PRIVATE /wd4251 /wd4275)
        endif()
    endif()
endif()

# *********************************************************************
# Package / Install Stuff
# *********************************************************************

install(DIRECTORY include/ DESTINATION include)

install(
    TARGETS {{snake_case_name}}
    EXPORT {{kebab_case_name}}-targets
    RUNTIME DESTINATION bin
    LIBRARY DESTINATION lib
    ARCHIVE DESTINATION lib
    INCLUDES
    DESTINATION include)

install(
    EXPORT {{kebab_case_name}}-targets
    FILE {{kebab_case_name}}-targets.cmake
    NAMESPACE {{snake_case_name}}::
    DESTINATION lib/cmake/{{kebab_case_name}})

file(
    WRITE "${PROJECT_BINARY_DIR}/{{kebab_case_name}}Config.cmake"
    "
include(CMakeFindDependencyMacro)
include(\\"\${CMAKE_CURRENT_LIST_DIR}/{{kebab_case_name}}-targets.cmake\\")
")

write_basic_package_version_file(
    "${PROJECT_BINARY_DIR}/{{kebab_case_name}}ConfigVersion.cmake"
    VERSION 1.0.1
    COMPATIBILITY AnyNewerVersion)

install(FILES "${PROJECT_BINARY_DIR}/{{kebab_case_name}}Config.cmake"
              "${PROJECT_BINARY_DIR}/{{kebab_case_name}}ConfigVersion.cmake"
        DESTINATION lib/cmake/{{kebab_case_name}})

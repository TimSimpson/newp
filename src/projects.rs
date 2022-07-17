// THIS CODE IS AUTO-GENERATED. DO NOT EDIT.
// Or, I dunno, you can if you want. Not like I can stop you really.

use std::collections::HashMap;

pub fn load_project_templates() -> HashMap<&'static str, HashMap<&'static str, &'static str>> {
    let mut result = HashMap::new();
    let mut project = HashMap::new();
    project.insert(r#".gitignore"#, r#"output/*"#);
    project.insert(
        r#"CMakeLists.txt"#,
        r#"# *********************************************************************
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
"#,
    );
    project.insert(
        r#"conanfile.py"#,
        r#"import os.path

import conans


class {{pascal_case_name}}(conans.ConanFile):
    name = "{{kebab_case_name}}"
    version = "0.1.0"
    license = "{{license}}"
    author = "{{author}}"
    description = "{{description}}"

    settings = "os", "compiler", "build_type", "arch"
    options = {"shared": [True, False]}
    default_options = {"shared": False}

    requires = tuple()

    build_requires = []

    test_requires = [
        "Catch2/2.11.1@catchorg/stable",
    ]

    @property
    def tests_enabled(self):
        return (
            self.develop
            and (os.environ.get("CONAN_SKIP_TESTS") or "").lower() != 'true'
        )

    def build_requirements(self):
        if self.tests_enabled:
            for tr in self.test_requires:
                self.build_requires(tr)

    generators = "cmake_find_package"

    exports_sources = (
        "src/*", "include/*", "demos/*", "tests/*", "CMakeLists.txt"
    )

    def _configed_cmake(self):
        cmake = conans.CMake(self)
        cmake.configure(defs={
            "{{scream_case_name}}_Build_Tests": self.tests_enabled,
        })
        return cmake

    def build(self):
        cmake = self._configed_cmake()
        cmake.build()

    def package(self):
        cmake = self._configed_cmake()
        cmake.install()

    def package_info(self):
        self.cpp_info.name = "{{kebab_case_name}}"
        self.cpp_info.libs = [ "{{snake_case_name}}" ]
"#,
    );
    project.insert(
        r#"demos/{{name}}_cli.cpp"#,
        r#"#include <iostream>
#include <{{name}}.hpp>

int main(int argc, char * * argv) {
    std::cout << "hi! The answer is " << {{snake_case_name}}::hello() << ".\\n";
    return 0;
}"#,
    );
    project.insert(
        r#"include/{{name}}.hpp"#,
        r#"#ifndef FILE_GUARD_{{scream_case_name}}_HPP
#define FILE_GUARD_{{scream_case_name}}_HPP
#pragma once

namespace {{snake_case_name}} {
    int hello();
}

#endif
"#,
    );
    project.insert(
        r#"README.md"#,
        r#"# {{name}}

{{description}}
"#,
    );
    project.insert(
        r#"src/{{name}}.cpp"#,
        r#"#include <{{name}}.hpp>


namespace {{snake_case_name}} {
    int hello() {
        return 42;
    }
}"#,
    );
    project.insert(
        r#"tests/{{name}}_test.cpp"#,
        r#"#include <iostream>
#include <{{name}}.hpp>

#define CATCH_CONFIG_MAIN
#include <catch2/catch.hpp>

TEST_CASE("Hello Tests", "[hello]") {
    const auto expected = 42;
    const auto actual = {{snake_case_name}}::hello();
    REQUIRE(expected == actual);
}"#,
    );
    project.insert(
        r#"__desc"#,
        r#"a typical C++ project using CMake and Conan"#,
    );
    result.insert(r#"cpp"#, project);
    let mut project = HashMap::new();
    project.insert(
        r#".gitignore"#,
        r#"lib
node_modules"#,
    );
    project.insert(
        r#".prettierignore"#,
        r#"lib
node_modules"#,
    );
    project.insert(
        r#".prettierrc.json"#,
        r#"{}
"#,
    );
    project.insert(
        r#"babel.config.json"#,
        r#"{
  "presets": ["@babel/preset-env", "@babel/preset-typescript"],
  "plugins": [
    "@babel/proposal-class-properties",
    "@babel/proposal-object-rest-spread",
    [
      "module-resolver",
      {
        "root": ["./src"]
      }
    ]
  ]
}
"#,
    );
    project.insert(
        r#"bin/{{camel_case_name}}.ts"#,
        r#"#!/usr/bin/env node

console.log("hi!");
"#,
    );
    project.insert(
        r#"Justfile"#,
        r#"# use with https://github.com/casey/just

_default:
    @just --list

# Sets up project (required for the other commands to work)
setup:
    npm run install

# Runs TypeScript and other checks
check:
    npm run check

# Runs tests
test:
    npm run test

# Transpiles code from `src` into standard Javascript in `lib`
build:
    npm run build

# Runs all checks in CI
fmt:
    npm run fmt

# Runs CI but locally (will reformat files for you)
ci-local:
    npm run ci-local

# Runs CI
ci:
    npm run ci"#,
    );
    project.insert(
        r#"package.json"#,
        r#"{
  "name": "{{camel_case_name}}",
  "version": "0.1.0",
  "description": "{{ description }}",
  "bin": {
    "{{ name }}": "./bin/{{ camel_case_name }}.js"
  },
  "scripts": {
    "build": "babel src --out-dir lib --extensions '.ts,.tsx'",
    "check": "tsc --project tsconfig.json --outDir output/tsOutput --noEmit",
    "ci": "npm run fmt-ci && npm run check && npm run test",
    "ci-local": "npm run fmt && npm run check && npm run test",
    "fmt": "prettier --write .",
    "fmt-ci": "prettier --check .",
    "run": "ts-node --transpile-only ./bin/{{ camel_case_name }}.ts",
    "test": "jest"
  },
  "author": "{{ author }}",
  "license": "{{ license }}",
  "jest": {
    "verbose": true,
    "modulePaths": [
      "src"
    ]
  },
  "devDependencies": {
    "@babel/cli": "^7.18.6",
    "@babel/core": "^7.18.6",
    "@babel/plugin-proposal-class-properties": "^7.18.6",
    "@babel/plugin-proposal-object-rest-spread": "^7.18.6",
    "@babel/preset-env": "^7.18.6",
    "@babel/preset-typescript": "^7.18.6",
    "@types/jest": "^26.0.24",
    "babel-jest": "^26.6.3",
    "babel-plugin-module-resolver": "^4.1.0",
    "jest": "^26.6.3",
    "prettier": "^2.1.1",
    "ts-jest": "^26.5.6",
    "ts-node": "^10.9.1",
    "typescript": "^4.7.4"
  }
}
"#,
    );
    project.insert(r#"README.md"#, r#"# {{name}}

{{description}}

## Development

Make sure NodeJS is installed. From this directory, run `npm install`. After that run any of the commands below:

`npm run check` - Runs TypeScript checks
`npm run test` - Runs tests
`npm run build` - Transpiles code from `src` into standard Javascript in `lib`
`npm run fmt` - Runs prettier
`npm run ci-local` - Runs all checks for CI, but will format your code for you
`npm run checks-ci` - Runs all checks for CI
"#);
    project.insert(
        r#"src/{{camel_case_name}}.ts"#,
        r#"export const hello = () => {
  return 42;
};
"#,
    );
    project.insert(
        r#"__tests__/test{{pascal_case_name}}.ts"#,
        r#"import { hello } from "{{ camel_case_name }}";

test("example test", () => {
  expect(hello()).toBe(42);
});"#,
    );
    project.insert(
        r#"tsconfig.json"#,
        r#"{
  "compilerOptions": {
    "skipLibCheck": true,
    "target": "es6",
    "noEmit": true,
    "noImplicitAny": true,
    "noImplicitThis": true,
    "strictNullChecks": true,
    "types": ["node", "jest"]
  },
  "exclude": ["node_modules"],
  "baseUrl": "/",
  "paths": {
    "*": ["src/*", "tests/*"]
  },
  "include": ["src/**/*.ts", "tests/**/*.ts"]
}
"#,
    );
    project.insert(r#"__desc"#, r#""#);
    result.insert(r#"javascript"#, project);
    let mut project = HashMap::new();
    project.insert(
        r#".gitignore"#,
        r#".mypy_cache
.pytest_cache
__pycache__
dist/*
{{ name }}.egg-info/*
"#,
    );
    project.insert(
        r#"mypy.ini"#,
        r#"[mypy]
python_version=3.6

check_untyped_defs=True
disallow_any_generics=True
disallow_untyped_calls=True
disallow_untyped_defs=True
follow_imports=normal
strict_optional=True
warn_no_return=True
warn_redundant_casts=True
warn_return_any=True
warn_unused_ignores=True"#,
    );
    project.insert(
        r#"pyproject.toml"#,
        r#"[tool.poetry]
name = "{{name}}"
version = "0.0.1"
description = ""
authors = [{{author}}]

[tool.poetry.dependencies]
python = "^3.6"
typing-extensions = "^3.7.4"

[tool.poetry.dev-dependencies]
black = "^19.10b0"
coverage = "^4.5.2"
flake8 = "^3.7.9"
flake8-bugbear = "^19.8.0"
mypy = "^0.761"
pytest = "^5.2"
taskipy = "^1.3.0"

[tool.black]
line-length = 80
target-version = ['py36', 'py37', 'py38']
include = '\.pyi?$'
exclude = '''
/(
    \.eggs
  | \.git
  | \.hg
  | \.mypy_cache
  | \.tox
  | \.venv
  | _build
  | buck-out
  | build
  | dist
)/
'''

[tool.pytest]
testpaths = [ '{{snake_case_name}}', 'tests' ]

[build-system]
requires = ["poetry>=0.12"]
build-backend = "poetry.masonry.api"

[tool.taskipy.tasks]
black = "black {{snake_case_name}} tests"
flake8 = "flake8  --extend-ignore=E203,E501 {{snake_case_name}} tests"
mypy = "mypy {{snake_case_name}} tests"
tests = "PYTHONPATH=. pytest -vv"
checks = 'task black && task flake8 && task mypy && task tests'

[tool.poetry.scripts]
{{snake_case_name}} = "{{snake_case_name}}.cli:main"

"#,
    );
    project.insert(
        r#"README.md"#,
        r#"# {{name}}

{{description}}

This project uses [poetry](https://python-poetry.org/). Install that, then run
the tests with:

```bash

  poetry install
  poetry run task checks

```"#,
    );
    project.insert(
        r#"{{snake_case_name}}/cli.py"#,
        r#"import argparse


def cli() -> None:
    parser = argparse.ArgumentParser(description={{description_quoted}})
    parser.add_argument("words", type=str, help="words to echo back")
    args = parser.parse_args()

    print(args.words)

"#,
    );
    project.insert(r#"{{snake_case_name}}/__init__.py"#, r#""#);
    project.insert(
        r#"tests/test_{{snake_case_name}}.py"#,
        r#"def test_example() -> None:
    assert 4 == 2 + 2
"#,
    );
    project.insert(r#"__desc"#, r#"a new-style Python project"#);
    result.insert(r#"python"#, project);
    result
}

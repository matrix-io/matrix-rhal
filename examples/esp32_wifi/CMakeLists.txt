# Build file used by `idf.py build` or when using cmake directly.
cmake_minimum_required(VERSION 3.5)
include($ENV{IDF_PATH}/tools/cmake/project.cmake)
# Binary output.
# Will create `build/PROJECT_NAME.bin`.
# If change this need to update:
# - `install.sh`
project(esp-app)

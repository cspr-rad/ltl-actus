# TODO: tests

CMAKE_MINIMUM_REQUIRED(VERSION 2.8)

# Name of the project
project(ltl-actus)

# Set default build type to Debug
if(NOT CMAKE_BUILD_TYPE)
  set(CMAKE_BUILD_TYPE Debug CACHE STRING "Choose the build type (Debug or Release)" FORCE)
endif()

message(STATUS "CMake build type: " ${CMAKE_BUILD_TYPE})

# Version of the project
set(PROJECT_VERSION "1.0.0")

# If build type is Debug, generate debugging information and don't optimise
if (CMAKE_BUILD_TYPE STREQUAL Debug)
  add_definitions(-O0)
  set(CMAKE_C_FLAGS "-g -Wall -pedantic")
else()
  add_definitions(-O3)
  set(CMAKE_C_FLAGS "-Wall -pedantic")
endif()

# CPack

set(CPACK_PACKAGE_CONTACT "quinn@casper.network")
set(CPACK_GENERATOR "DEB;RPM;ZIP;TGZ")
set(CPACK_PACKAGE_DESCRIPTION, "Linear temporal logic to execute the algorithmic contract types unified standard")
set(CPACK_PACKAGE_VERSION ${PROJECT_VERSION})

# DEB package
set(CPACK_DEBIAN_PACKAGE_HOMEPAGE "<project website>")

# DEB package dependencies e.g. libncurses5 (>= 5.0), libglib2.0-0 (>= 2.0)
# List of dependencies for a DEB package
# E.g.: libncurses5 (>= 5.0), libglib2.0-0 (>= 2.0)
set(CPACK_DEBIAN_PACKAGE_DEPENDS "")

# RPM package
set(CPACK_RPM_PACKAGE_LICENSE "mit")
set(CPACK_RPM_PACKAGE_URL "<project website>")
set(CPACK_RPM_PACKAGE_DESCRIPTION "Linear temporal logic to execute the algorithmic contract types unified standard")

# List of dependencies for an RPM package
# E.g.: ncurses >= 5.0, glib2 >= 2.0
set(CPACK_RPM_PACKAGE_REQUIRES "")

include(CPack)

# End CPack
# C standard for the compiler to use
set(CMAKE_C_STANDARD 17)

# Source code files, all .h and .c files from src/ directory are included
file(GLOB SOURCES "include/*.h" "src/*.c")

# Executable name
add_executable(${PROJECT_NAME} ${SOURCES})


############################################
#                                          #
#              Dependencies                #
#                                          #
############################################

find_package(PkgConfig)

pkg_check_modules(PKG_CONFIG "pkg-config")

pkg_check_modules(GLIB "glib-2.0")
include_directories(
  ${GLIB_INCLUDE_DIRS}
  )

link_directories(
  ${GLIB_LIBRARY_DIRS}
  )

add_definitions(
  ${GLIB_CFLAGS_OTHER}
  )

target_link_libraries(${PROJECT_NAME}
  ${GLIB_LIBRARIES}
  )


############################################
#                                          #
#              Installation                #
#                                          #
############################################

install(TARGETS ${PROJECT_NAME} DESTINATION bin)

install(FILES ltl-actus.1 DESTINATION share/man/man1)

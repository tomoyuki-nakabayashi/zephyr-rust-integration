cmake_minimum_required(VERSION 3.13.1)

include($ENV{ZEPHYR_BASE}/cmake/app/boilerplate.cmake NO_POLICY_SCOPE)
project(hello_world)

target_sources(app PRIVATE src/main.c)

include(ExternalProject)

set(rust_src_dir   ${CMAKE_CURRENT_SOURCE_DIR}/hello)
set(rust_build_dir ${CMAKE_CURRENT_BINARY_DIR}/hello)

ExternalProject_Add(
  rust_hello                   # Name for custom target
  PREFIX     ${rust_build_dir} # Root dir for entire project
  SOURCE_DIR ${rust_src_dir}
  BINARY_DIR ${rust_src_dir}   # This particular build system is invoked from the root
  CONFIGURE_COMMAND ""         # Skip configuring the project, e.g. with autoconf
  BUILD_COMMAND make
  INSTALL_COMMAND ""      # This particular build system has no install command
  BUILD_BYPRODUCTS ${rust_src_dir}/lib/librust.a
)

add_library(rust_hello_lib STATIC IMPORTED GLOBAL)
add_dependencies(
  rust_hello_lib
  rust_hello
)

set_target_properties(rust_hello_lib PROPERTIES IMPORTED_LOCATION ${rust_src_dir}/lib/librust.a)
set_target_properties(rust_hello_lib PROPERTIES INTERFACE_INCLUDE_DIRECTORIES ${rust_src_dir}/lib)

target_link_libraries(app PUBLIC rust_hello_lib)
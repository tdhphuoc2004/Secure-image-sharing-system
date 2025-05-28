#----------------------------------------------------------------
# Generated CMake target import file for configuration "RelWithDebInfo".
#----------------------------------------------------------------

# Commands may need to know the format version.
set(CMAKE_IMPORT_FILE_VERSION 1)

# Import target "AWS::crypto" for configuration "RelWithDebInfo"
set_property(TARGET AWS::crypto APPEND PROPERTY IMPORTED_CONFIGURATIONS RELWITHDEBINFO)
set_target_properties(AWS::crypto PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_RELWITHDEBINFO "ASM_NASM;C"
  IMPORTED_LOCATION_RELWITHDEBINFO "${_IMPORT_PREFIX}/lib/aws_lc_0_29_0_crypto.lib"
  )

list(APPEND _cmake_import_check_targets AWS::crypto )
list(APPEND _cmake_import_check_files_for_AWS::crypto "${_IMPORT_PREFIX}/lib/aws_lc_0_29_0_crypto.lib" )

# Commands beyond this point should not need to know the version.
set(CMAKE_IMPORT_FILE_VERSION)

#Copyright (C) 2016 Chris Liebert

CMAKE_MINIMUM_REQUIRED(VERSION 3.0)

SET(QUICK3D_LIB_PATH "${CMAKE_CURRENT_SOURCE_DIR}")

FIND_PACKAGE(SWIG REQUIRED)
INCLUDE(${SWIG_USE_FILE})

INCLUDE_DIRECTORIES(${CMAKE_CURRENT_SOURCE_DIR})

LINK_DIRECTORIES(${QUICK3D_LIB_PATH})

SET(CMAKE_BUILD_TYPE Debug)
SET(CMAKE_SWIG_FLAGS "")

SWIG_ADD_MODULE(quick3dwrapper csharp quick3d.i)

SET(QUICK3D_LIBRARY quick3d)

IF(MSVC)
	SET(QUICK3D_LIBRARY quick3d opengl32 user32 gdi32 dwmapi dbghelp shell32 advapi32 msvcrt kernel32 ws2_32 userenv shell32 advapi32)
	# LINK_DIRECTORIES(..) alone is not sufficient for MSVC
	SET(MSVCLIBPATH "${CMAKE_EXE_LINKER_FLAGS} /LIBPATH:${QUICK3D_LIB_PATH}")
	SET(CMAKE_EXE_LINKER_FLAGS "${MSVCLIBPATH}")
	SWIG_LINK_LIBRARIES(quick3dwrapper ${QUICK3D_LIBRARY})
ELSE(MSVC)
	SWIG_LINK_LIBRARIES(quick3dwrapper ${QUICK3D_LIBRARY})
ENDIF(MSVC)

# Use CXX instead of C
SET_TARGET_PROPERTIES(quick3dwrapper PROPERTIES LINKER_LANGUAGE CXX)

# This will copy quick3dwrapper.[dll/so] to the parent directory and is used in build.rs
INSTALL(TARGETS quick3dwrapper DESTINATION ${CMAKE_CURRENT_SOURCE_DIR}/..)

# When SWIG generates a C# wrapper, there are C# files generated in addition to the native code
# This needs to be accessable in order to use the native libraries
# This will copy to a directory called build
INSTALL(
	DIRECTORY
	${CMAKE_BINARY_DIR}
	DESTINATION
	${CMAKE_CURRENT_SOURCE_DIR}/..
	FILES_MATCHING
	PATTERN
	"*.cs"
)

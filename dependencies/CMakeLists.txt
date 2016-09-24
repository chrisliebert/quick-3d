# Copyright (C) 2016 Chris Liebert

CMAKE_MINIMUM_REQUIRED(VERSION 2.8)

PROJECT(quick3d-dependencies)

SET(CMAKE_BUILD_TYPE Debug)

# Dummy stdc++ library for MSVC
# Rust MSVC does not need to be linked with stdc++
IF(MSVC)
	ADD_LIBRARY(stdc++ support/msvc-stdc++.cpp)
ENDIF(MSVC)
	
INCLUDE_DIRECTORIES(${CMAKE_CURRENT_SOURCE_DIR})

SET(SQLITE3_LIBRARY sqlite3)
SET(SQLITE3_LIBRARIES sqlite3)

IF(MSVC)
	set(MSVCLIBPATH "${CMAKE_EXE_LINKER_FLAGS} /LIBPATH:${CMAKE_CURRENT_SOURCE_DIR}")
	set(CMAKE_EXE_LINKER_FLAGS "${MSVCLIBPATH}")
ELSEIF(MSVC)
	SET(SQLITE3_LIBRARIES sqlite3 m)
ENDIF(MSVC)

INCLUDE_DIRECTORIES(${CMAKE_CURRENT_SOURCE_DIR}/sqlite3)

ADD_LIBRARY(${SQLITE3_LIBRARY} STATIC
	sqlite3/sqlite3.c
	sqlite3/sqlite3.h
)

IF(MSVC)
	TARGET_LINK_LIBRARIES(sqlite3)
ELSEIF(UNIX)
	FIND_PACKAGE (Threads REQUIRED)
	
	TARGET_LINK_LIBRARIES(sqlite3 dl m ${CMAKE_THREAD_LIBS_INIT})
ELSE(MSVC)
	TARGET_LINK_LIBRARIES(sqlite3 m)
ENDIF(MSVC)

INCLUDE_DIRECTORIES(${CMAKE_CURRENT_SOURCE_DIR}/tinyobjloader)
ADD_LIBRARY(tinyobjloader STATIC
	tinyobjloader/tiny_obj_loader.cc
	tinyobjloader/tiny_obj_loader.h
)

INCLUDE_DIRECTORIES(${CMAKE_CURRENT_SOURCE_DIR}/obj2sqlite)

ADD_LIBRARY(scenebuilder
	obj2sqlite/ConfigLoader.cpp
	obj2sqlite/ConfigLoader.h
	obj2sqlite/SceneBuilder.cpp
	obj2sqlite/SceneBuilder.h
)

ADD_EXECUTABLE(obj2sqlite
	obj2sqlite/obj2sqlite.cpp
)

LINK_DIRECTORIES(${CMAKE_CURRENT_SOURCE_DIR})

TARGET_LINK_LIBRARIES(scenebuilder tinyobjloader ${SQLITE3_LIBRARIES})
TARGET_LINK_LIBRARIES(obj2sqlite scenebuilder)

INSTALL(TARGETS ${SQLITE3_LIBRARY} tinyobjloader scenebuilder DESTINATION ${CMAKE_CURRENT_SOURCE_DIR})

# Install obj2sqlite binary to parent directory
INSTALL(TARGETS obj2sqlite DESTINATION ${CMAKE_CURRENT_SOURCE_DIR}/..)

# Install stdc++.lib on MSVC
IF(MSVC)
	INSTALL(TARGETS stdc++ DESTINATION ${CMAKE_CURRENT_SOURCE_DIR})
ENDIF(MSVC)


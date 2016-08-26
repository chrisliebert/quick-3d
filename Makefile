# Copyright (C) 2016 Chris Liebert

LUA_INCLUDE_DIR=/usr/include/luajit-2.0
LUA_LIBRARY_DIR=/usr/lib/x86_64-linux-gnu
LUA_LIB=luajit-5.1

REMOVE=rm -f

QUICK3D_LIBRARY_DIR=target/debug
QUICK3D_LIB=${QUICK3D_LIBRARY_DIR}/libquick3d.so
QUICK3D_WRAPPER_LIB=quick3dwrapper.so

JNI_CFLAGS=-I/usr/lib/jvm/java-8-oracle/include -I/usr/lib/jvm/java-8-oracle/include/linux

PY_INCLUDE_DIR=/usr/include/python2.7

CFLAGS=-fPIC
LDFLAGS=-l${LUA_LIB}

all: lualib c_example
	echo "Built Quick3D"

lualib: quick3d swig_lua_wrapper
	gcc quick3d_wrap.c ${QUICK3D_LIB} ${CFLAGS} -I${LUA_INCLUDE_DIR} -L${LUA_LIBRARY_DIR} -shared -o ${QUICK3D_WRAPPER_LIB}

jnilib: quick3d swig_jni_wrapper
	gcc quick3d_wrap.c ${QUICK3D_LIB} ${CFLAGS} ${JNI_CFLAGS} -shared -o lib${QUICK3D_WRAPPER_LIB}
	javac Example.java

pylib: quick3d swig_py_wrapper
	gcc quick3d_wrap.c ${QUICK3D_LIB} ${CFLAGS} -I${PY_INCLUDE_DIR} -shared -o _${QUICK3D_WRAPPER_LIB}

quick3d:
	cargo build

swig_lua_wrapper:
	${REMOVE} quick3d_wrap.c
	swig -lua quick3d.h

swig_jni_wrapper:
	${REMOVE} quick3d_wrap.c
	swig -java quick3d.h

swig_py_wrapper:
	${REMOVE} quick3d_wrap.c	
	swig -python quick3d.h

c_example:
	gcc example.c ${QUICK3D_LIB} -o c_example

clean:
	${REMOVE} quick3d_wrap.c ${QUICK3D_LIB} ${QUICK3D_WRAPPER_LIB} c_example
	cargo clean


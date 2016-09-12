# Copyright (C) 2016 Chris Liebert

REMOVE=rm -f

QUICK3D_LIBRARY_DIR=target/debug
QUICK3D_LIB=${QUICK3D_LIBRARY_DIR}/libquick3d.so
QUICK3D_WRAPPER_LIB=quick3dwrapper.so

PY_INCLUDE_DIR=/usr/include/python2.7

CFLAGS=-fPIC

all: pylib
	echo "Built Wrapper"

pylib: swig_py_wrapper
	cp ${QUICK3D_LIB} .
	gcc quick3d_wrap.c ${QUICK3D_LIB} ${CFLAGS} -I${PY_INCLUDE_DIR} -shared -o _${QUICK3D_WRAPPER_LIB}

swig_py_wrapper:
	${REMOVE} quick3d_wrap.c	
	swig -python quick3d.h

clean:
	${REMOVE} quick3d_wrap.c ${QUICK3D_LIB} ${QUICK3D_WRAPPER_LIB}


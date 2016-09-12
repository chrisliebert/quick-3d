# Copyright (C) 2016 Chris Liebert

REMOVE=rm -f

QUICK3D_LIBRARY_DIR=../../target/debug
QUICK3D_LIB=${QUICK3D_LIBRARY_DIR}/libquick3d.so
QUICK3D_WRAPPER_LIB=quick3dwrapper.so

JNI_CFLAGS=-I/usr/lib/jvm/java-8-oracle/include -I/usr/lib/jvm/java-8-oracle/include/linux

CFLAGS=-fPIC

all: jnilib
	echo "Built Wrapper"

jnilib: swig_jni_wrapper
	cp ${QUICK3D_LIB} .
	gcc quick3d_wrap.c ${QUICK3D_LIB} ${CFLAGS} ${JNI_CFLAGS} -shared -o lib${QUICK3D_WRAPPER_LIB}
	javac Example.java

swig_jni_wrapper:
	cp ../../quick3d.i .
	${REMOVE} quick3d_wrap.c
	swig -java quick3d.i

clean:
	${REMOVE} quick3d_wrap.c ${QUICK3D_LIB} ${QUICK3D_WRAPPER_LIB} c_example
	cargo clean

/* Copyright (C) 2016 Chris Liebert */

#ifndef _QUICK3D_I_
#define _QUICK3D_I_

#ifdef SWIG

 %module quick3dwrapper

 %{
  #include "quick3d.i"
  #include <stdbool.h>
  #include <stdint.h>
  #include "quick3d.h"
 %}

 %include "quick3d.h"

#endif /* SWIG */

#endif /* _QUICK3D_I_ */

# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/cmpsf-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/cmpsf-1.c" 2


# 1 "/usr/lib/llvm-18/lib/clang/18/include/limits.h" 1 3
# 21 "/usr/lib/llvm-18/lib/clang/18/include/limits.h" 3
# 1 "/usr/include/limits.h" 1 3 4
# 26 "/usr/include/limits.h" 3 4
# 1 "/usr/include/x86_64-linux-gnu/bits/libc-header-start.h" 1 3 4
# 33 "/usr/include/x86_64-linux-gnu/bits/libc-header-start.h" 3 4
# 1 "/usr/include/features.h" 1 3 4
# 394 "/usr/include/features.h" 3 4
# 1 "/usr/include/features-time64.h" 1 3 4
# 20 "/usr/include/features-time64.h" 3 4
# 1 "/usr/include/x86_64-linux-gnu/bits/wordsize.h" 1 3 4
# 21 "/usr/include/features-time64.h" 2 3 4
# 1 "/usr/include/x86_64-linux-gnu/bits/timesize.h" 1 3 4
# 19 "/usr/include/x86_64-linux-gnu/bits/timesize.h" 3 4
# 1 "/usr/include/x86_64-linux-gnu/bits/wordsize.h" 1 3 4
# 20 "/usr/include/x86_64-linux-gnu/bits/timesize.h" 2 3 4
# 22 "/usr/include/features-time64.h" 2 3 4
# 395 "/usr/include/features.h" 2 3 4
# 480 "/usr/include/features.h" 3 4
# 1 "/usr/include/stdc-predef.h" 1 3 4
# 481 "/usr/include/features.h" 2 3 4
# 502 "/usr/include/features.h" 3 4
# 1 "/usr/include/x86_64-linux-gnu/sys/cdefs.h" 1 3 4
# 576 "/usr/include/x86_64-linux-gnu/sys/cdefs.h" 3 4
# 1 "/usr/include/x86_64-linux-gnu/bits/wordsize.h" 1 3 4
# 577 "/usr/include/x86_64-linux-gnu/sys/cdefs.h" 2 3 4
# 1 "/usr/include/x86_64-linux-gnu/bits/long-double.h" 1 3 4
# 578 "/usr/include/x86_64-linux-gnu/sys/cdefs.h" 2 3 4
# 503 "/usr/include/features.h" 2 3 4
# 526 "/usr/include/features.h" 3 4
# 1 "/usr/include/x86_64-linux-gnu/gnu/stubs.h" 1 3 4
# 10 "/usr/include/x86_64-linux-gnu/gnu/stubs.h" 3 4
# 1 "/usr/include/x86_64-linux-gnu/gnu/stubs-64.h" 1 3 4
# 11 "/usr/include/x86_64-linux-gnu/gnu/stubs.h" 2 3 4
# 527 "/usr/include/features.h" 2 3 4
# 34 "/usr/include/x86_64-linux-gnu/bits/libc-header-start.h" 2 3 4
# 27 "/usr/include/limits.h" 2 3 4
# 195 "/usr/include/limits.h" 3 4
# 1 "/usr/include/x86_64-linux-gnu/bits/posix1_lim.h" 1 3 4
# 27 "/usr/include/x86_64-linux-gnu/bits/posix1_lim.h" 3 4
# 1 "/usr/include/x86_64-linux-gnu/bits/wordsize.h" 1 3 4
# 28 "/usr/include/x86_64-linux-gnu/bits/posix1_lim.h" 2 3 4
# 161 "/usr/include/x86_64-linux-gnu/bits/posix1_lim.h" 3 4
# 1 "/usr/include/x86_64-linux-gnu/bits/local_lim.h" 1 3 4
# 38 "/usr/include/x86_64-linux-gnu/bits/local_lim.h" 3 4
# 1 "/usr/include/linux/limits.h" 1 3 4
# 39 "/usr/include/x86_64-linux-gnu/bits/local_lim.h" 2 3 4
# 81 "/usr/include/x86_64-linux-gnu/bits/local_lim.h" 3 4
# 1 "/usr/include/x86_64-linux-gnu/bits/pthread_stack_min-dynamic.h" 1 3 4
# 29 "/usr/include/x86_64-linux-gnu/bits/pthread_stack_min-dynamic.h" 3 4
# 1 "/usr/include/x86_64-linux-gnu/bits/pthread_stack_min.h" 1 3 4
# 30 "/usr/include/x86_64-linux-gnu/bits/pthread_stack_min-dynamic.h" 2 3 4
# 82 "/usr/include/x86_64-linux-gnu/bits/local_lim.h" 2 3 4
# 162 "/usr/include/x86_64-linux-gnu/bits/posix1_lim.h" 2 3 4
# 196 "/usr/include/limits.h" 2 3 4



# 1 "/usr/include/x86_64-linux-gnu/bits/posix2_lim.h" 1 3 4
# 200 "/usr/include/limits.h" 2 3 4
# 22 "/usr/lib/llvm-18/lib/clang/18/include/limits.h" 2 3
# 4 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/cmpsf-1.c" 2




feq (float x, float y)
{
  if (x == y)
    return 13;
  else
    return 140;
}

fne (float x, float y)
{
  if (x != y)
    return 13;
  else
    return 140;
}

flt (float x, float y)
{
  if (x < y)
    return 13;
  else
    return 140;
}

fge (float x, float y)
{
  if (x >= y)
    return 13;
  else
    return 140;
}

fgt (float x, float y)
{
  if (x > y)
    return 13;
  else
    return 140;
}

fle (float x, float y)
{
  if (x <= y)
    return 13;
  else
    return 140;
}

float args[] =
{
  0.0F,
  1.0F,
  -1.0F,
  3.40282347e+38F,
  1.17549435e-38F,
  0.0000000000001F,
  123456789.0F,
  -987654321.0F
};

int correct_results[] =
{
 13, 140, 140, 13, 140, 13,
 140, 13, 13, 140, 140, 13,
 140, 13, 140, 13, 13, 140,
 140, 13, 13, 140, 140, 13,
 140, 13, 13, 140, 140, 13,
 140, 13, 13, 140, 140, 13,
 140, 13, 13, 140, 140, 13,
 140, 13, 140, 13, 13, 140,
 140, 13, 140, 13, 13, 140,
 13, 140, 140, 13, 140, 13,
 140, 13, 140, 13, 13, 140,
 140, 13, 13, 140, 140, 13,
 140, 13, 140, 13, 13, 140,
 140, 13, 140, 13, 13, 140,
 140, 13, 13, 140, 140, 13,
 140, 13, 140, 13, 13, 140,
 140, 13, 13, 140, 140, 13,
 140, 13, 13, 140, 140, 13,
 13, 140, 140, 13, 140, 13,
 140, 13, 13, 140, 140, 13,
 140, 13, 13, 140, 140, 13,
 140, 13, 13, 140, 140, 13,
 140, 13, 13, 140, 140, 13,
 140, 13, 140, 13, 13, 140,
 140, 13, 140, 13, 13, 140,
 140, 13, 140, 13, 13, 140,
 140, 13, 140, 13, 13, 140,
 13, 140, 140, 13, 140, 13,
 140, 13, 140, 13, 13, 140,
 140, 13, 140, 13, 13, 140,
 140, 13, 140, 13, 13, 140,
 140, 13, 140, 13, 13, 140,
 140, 13, 140, 13, 13, 140,
 140, 13, 13, 140, 140, 13,
 140, 13, 140, 13, 13, 140,
 140, 13, 13, 140, 140, 13,
 13, 140, 140, 13, 140, 13,
 140, 13, 13, 140, 140, 13,
 140, 13, 13, 140, 140, 13,
 140, 13, 140, 13, 13, 140,
 140, 13, 140, 13, 13, 140,
 140, 13, 13, 140, 140, 13,
 140, 13, 140, 13, 13, 140,
 140, 13, 13, 140, 140, 13,
 140, 13, 140, 13, 13, 140,
 13, 140, 140, 13, 140, 13,
 140, 13, 13, 140, 140, 13,
 140, 13, 140, 13, 13, 140,
 140, 13, 140, 13, 13, 140,
 140, 13, 140, 13, 13, 140,
 140, 13, 140, 13, 13, 140,
 140, 13, 13, 140, 140, 13,
 140, 13, 140, 13, 13, 140,
 140, 13, 140, 13, 13, 140,
 13, 140, 140, 13, 140, 13,
 140, 13, 140, 13, 13, 140,
 140, 13, 13, 140, 140, 13,
 140, 13, 13, 140, 140, 13,
 140, 13, 13, 140, 140, 13,
 140, 13, 13, 140, 140, 13,
 140, 13, 13, 140, 140, 13,
 140, 13, 13, 140, 140, 13,
 140, 13, 13, 140, 140, 13,
 13, 140, 140, 13, 140, 13,
};

int
main (void)
{
  int i, j, *res = correct_results;

  for (i = 0; i < 8; i++)
    {
      float arg0 = args[i];
      for (j = 0; j < 8; j++)
 {
   float arg1 = args[j];

   if (feq (arg0, arg1) != *res++)
     abort ();
   if (fne (arg0, arg1) != *res++)
     abort ();
   if (flt (arg0, arg1) != *res++)
     abort ();
   if (fge (arg0, arg1) != *res++)
     abort ();
   if (fgt (arg0, arg1) != *res++)
     abort ();
   if (fle (arg0, arg1) != *res++)
     abort ();
 }
    }
  exit (0);
}

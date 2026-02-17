# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20020226-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20020226-1.c" 2


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
# 4 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20020226-1.c" 2

void abort (void);
void exit (int);
# 24 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20020226-1.c"
unsigned char uc = ((unsigned char)0x1234U);
unsigned short us = ((unsigned short)0x1234U);
unsigned int ui = 0x1234U;
unsigned long ul = 0x12345678LU;
unsigned long long ull = 0x12345678abcdef0LLU;
int shift1 = 4;
int shift2 = ((sizeof (long long) * 8) - 4);

int
main (void)
{
  if ((((uc) >> (shift1)) | ((uc) << ((sizeof (uc) * 8) - (shift1)))) != (((((unsigned char)0x1234U)) >> (4)) | ((((unsigned char)0x1234U)) << ((sizeof (((unsigned char)0x1234U)) * 8) - (4)))))
    abort ();

  if ((((uc) >> (4)) | ((uc) << ((sizeof (uc) * 8) - (4)))) != (((((unsigned char)0x1234U)) >> (4)) | ((((unsigned char)0x1234U)) << ((sizeof (((unsigned char)0x1234U)) * 8) - (4)))))
    abort ();

  if ((((us) >> (shift1)) | ((us) << ((sizeof (us) * 8) - (shift1)))) != (((((unsigned short)0x1234U)) >> (4)) | ((((unsigned short)0x1234U)) << ((sizeof (((unsigned short)0x1234U)) * 8) - (4)))))
    abort ();

  if ((((us) >> (4)) | ((us) << ((sizeof (us) * 8) - (4)))) != (((((unsigned short)0x1234U)) >> (4)) | ((((unsigned short)0x1234U)) << ((sizeof (((unsigned short)0x1234U)) * 8) - (4)))))
    abort ();

  if ((((ui) >> (shift1)) | ((ui) << ((sizeof (ui) * 8) - (shift1)))) != (((0x1234U) >> (4)) | ((0x1234U) << ((sizeof (0x1234U) * 8) - (4)))))
    abort ();

  if ((((ui) >> (4)) | ((ui) << ((sizeof (ui) * 8) - (4)))) != (((0x1234U) >> (4)) | ((0x1234U) << ((sizeof (0x1234U) * 8) - (4)))))
    abort ();

  if ((((ul) >> (shift1)) | ((ul) << ((sizeof (ul) * 8) - (shift1)))) != (((0x12345678LU) >> (4)) | ((0x12345678LU) << ((sizeof (0x12345678LU) * 8) - (4)))))
    abort ();

  if ((((ul) >> (4)) | ((ul) << ((sizeof (ul) * 8) - (4)))) != (((0x12345678LU) >> (4)) | ((0x12345678LU) << ((sizeof (0x12345678LU) * 8) - (4)))))
    abort ();

  if ((((ull) >> (shift1)) | ((ull) << ((sizeof (ull) * 8) - (shift1)))) != (((0x12345678abcdef0LLU) >> (4)) | ((0x12345678abcdef0LLU) << ((sizeof (0x12345678abcdef0LLU) * 8) - (4)))))
    abort ();

  if ((((ull) >> (4)) | ((ull) << ((sizeof (ull) * 8) - (4)))) != (((0x12345678abcdef0LLU) >> (4)) | ((0x12345678abcdef0LLU) << ((sizeof (0x12345678abcdef0LLU) * 8) - (4)))))
    abort ();

  if ((((ull) >> (shift2)) | ((ull) << ((sizeof (ull) * 8) - (shift2)))) != (((0x12345678abcdef0LLU) >> (((sizeof (long long) * 8) - 4))) | ((0x12345678abcdef0LLU) << ((sizeof (0x12345678abcdef0LLU) * 8) - (((sizeof (long long) * 8) - 4))))))
    abort ();

  if ((((ull) >> (((sizeof (long long) * 8) - 4))) | ((ull) << ((sizeof (ull) * 8) - (((sizeof (long long) * 8) - 4))))) != (((0x12345678abcdef0LLU) >> (((sizeof (long long) * 8) - 4))) | ((0x12345678abcdef0LLU) << ((sizeof (0x12345678abcdef0LLU) * 8) - (((sizeof (long long) * 8) - 4))))))
    abort ();

  if ((((uc) << (shift1)) | ((uc) >> ((sizeof (uc) * 8) - (shift1)))) != (((((unsigned char)0x1234U)) << (4)) | ((((unsigned char)0x1234U)) >> ((sizeof (((unsigned char)0x1234U)) * 8) - (4)))))
    abort ();

  if ((((uc) << (4)) | ((uc) >> ((sizeof (uc) * 8) - (4)))) != (((((unsigned char)0x1234U)) << (4)) | ((((unsigned char)0x1234U)) >> ((sizeof (((unsigned char)0x1234U)) * 8) - (4)))))
    abort ();

  if ((((us) << (shift1)) | ((us) >> ((sizeof (us) * 8) - (shift1)))) != (((((unsigned short)0x1234U)) << (4)) | ((((unsigned short)0x1234U)) >> ((sizeof (((unsigned short)0x1234U)) * 8) - (4)))))
    abort ();

  if ((((us) << (4)) | ((us) >> ((sizeof (us) * 8) - (4)))) != (((((unsigned short)0x1234U)) << (4)) | ((((unsigned short)0x1234U)) >> ((sizeof (((unsigned short)0x1234U)) * 8) - (4)))))
    abort ();

  if ((((ui) << (shift1)) | ((ui) >> ((sizeof (ui) * 8) - (shift1)))) != (((0x1234U) << (4)) | ((0x1234U) >> ((sizeof (0x1234U) * 8) - (4)))))
    abort ();

  if ((((ui) << (4)) | ((ui) >> ((sizeof (ui) * 8) - (4)))) != (((0x1234U) << (4)) | ((0x1234U) >> ((sizeof (0x1234U) * 8) - (4)))))
    abort ();

  if ((((ul) << (shift1)) | ((ul) >> ((sizeof (ul) * 8) - (shift1)))) != (((0x12345678LU) << (4)) | ((0x12345678LU) >> ((sizeof (0x12345678LU) * 8) - (4)))))
    abort ();

  if ((((ul) << (4)) | ((ul) >> ((sizeof (ul) * 8) - (4)))) != (((0x12345678LU) << (4)) | ((0x12345678LU) >> ((sizeof (0x12345678LU) * 8) - (4)))))
    abort ();

  if ((((ull) << (shift1)) | ((ull) >> ((sizeof (ull) * 8) - (shift1)))) != (((0x12345678abcdef0LLU) << (4)) | ((0x12345678abcdef0LLU) >> ((sizeof (0x12345678abcdef0LLU) * 8) - (4)))))
    abort ();

  if ((((ull) << (4)) | ((ull) >> ((sizeof (ull) * 8) - (4)))) != (((0x12345678abcdef0LLU) << (4)) | ((0x12345678abcdef0LLU) >> ((sizeof (0x12345678abcdef0LLU) * 8) - (4)))))
    abort ();

  if ((((ull) << (shift2)) | ((ull) >> ((sizeof (ull) * 8) - (shift2)))) != (((0x12345678abcdef0LLU) << (((sizeof (long long) * 8) - 4))) | ((0x12345678abcdef0LLU) >> ((sizeof (0x12345678abcdef0LLU) * 8) - (((sizeof (long long) * 8) - 4))))))
    abort ();

  if ((((ull) << (((sizeof (long long) * 8) - 4))) | ((ull) >> ((sizeof (ull) * 8) - (((sizeof (long long) * 8) - 4))))) != (((0x12345678abcdef0LLU) << (((sizeof (long long) * 8) - 4))) | ((0x12345678abcdef0LLU) >> ((sizeof (0x12345678abcdef0LLU) * 8) - (((sizeof (long long) * 8) - 4))))))
    abort ();

  exit (0);
}

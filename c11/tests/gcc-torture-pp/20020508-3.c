# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20020508-3.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20020508-3.c" 2
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
# 2 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20020508-3.c" 2

void abort (void);
void exit (int);
# 22 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20020508-3.c"
char c = ((char)0xf234);
short s = ((short)0xf234);
int i = ((int)0xf234);
long l = ((long)0xf2345678L);
long long ll = ((long long)0xf2345678abcdef0LL);
int shift1 = 4;
int shift2 = ((sizeof (long long) * 8) - 4);

int
main (void)
{
  if ((((c) >> (shift1)) | ((c) << ((sizeof (c) * 8) - (shift1)))) != (((((char)0xf234)) >> (4)) | ((((char)0xf234)) << ((sizeof (((char)0xf234)) * 8) - (4)))))
    abort ();

  if ((((c) >> (4)) | ((c) << ((sizeof (c) * 8) - (4)))) != (((((char)0xf234)) >> (4)) | ((((char)0xf234)) << ((sizeof (((char)0xf234)) * 8) - (4)))))
    abort ();

  if ((((s) >> (shift1)) | ((s) << ((sizeof (s) * 8) - (shift1)))) != (((((short)0xf234)) >> (4)) | ((((short)0xf234)) << ((sizeof (((short)0xf234)) * 8) - (4)))))
    abort ();

  if ((((s) >> (4)) | ((s) << ((sizeof (s) * 8) - (4)))) != (((((short)0xf234)) >> (4)) | ((((short)0xf234)) << ((sizeof (((short)0xf234)) * 8) - (4)))))
    abort ();

  if ((((i) >> (shift1)) | ((i) << ((sizeof (i) * 8) - (shift1)))) != (((((int)0xf234)) >> (4)) | ((((int)0xf234)) << ((sizeof (((int)0xf234)) * 8) - (4)))))
    abort ();

  if ((((i) >> (4)) | ((i) << ((sizeof (i) * 8) - (4)))) != (((((int)0xf234)) >> (4)) | ((((int)0xf234)) << ((sizeof (((int)0xf234)) * 8) - (4)))))
    abort ();

  if ((((l) >> (shift1)) | ((l) << ((sizeof (l) * 8) - (shift1)))) != (((((long)0xf2345678L)) >> (4)) | ((((long)0xf2345678L)) << ((sizeof (((long)0xf2345678L)) * 8) - (4)))))
    abort ();

  if ((((l) >> (4)) | ((l) << ((sizeof (l) * 8) - (4)))) != (((((long)0xf2345678L)) >> (4)) | ((((long)0xf2345678L)) << ((sizeof (((long)0xf2345678L)) * 8) - (4)))))
    abort ();

  if ((((ll) >> (shift1)) | ((ll) << ((sizeof (ll) * 8) - (shift1)))) != (((((long long)0xf2345678abcdef0LL)) >> (4)) | ((((long long)0xf2345678abcdef0LL)) << ((sizeof (((long long)0xf2345678abcdef0LL)) * 8) - (4)))))
    abort ();

  if ((((ll) >> (4)) | ((ll) << ((sizeof (ll) * 8) - (4)))) != (((((long long)0xf2345678abcdef0LL)) >> (4)) | ((((long long)0xf2345678abcdef0LL)) << ((sizeof (((long long)0xf2345678abcdef0LL)) * 8) - (4)))))
    abort ();

  if ((((ll) >> (shift2)) | ((ll) << ((sizeof (ll) * 8) - (shift2)))) != (((((long long)0xf2345678abcdef0LL)) >> (((sizeof (long long) * 8) - 4))) | ((((long long)0xf2345678abcdef0LL)) << ((sizeof (((long long)0xf2345678abcdef0LL)) * 8) - (((sizeof (long long) * 8) - 4))))))
    abort ();

  if ((((ll) >> (((sizeof (long long) * 8) - 4))) | ((ll) << ((sizeof (ll) * 8) - (((sizeof (long long) * 8) - 4))))) != (((((long long)0xf2345678abcdef0LL)) >> (((sizeof (long long) * 8) - 4))) | ((((long long)0xf2345678abcdef0LL)) << ((sizeof (((long long)0xf2345678abcdef0LL)) * 8) - (((sizeof (long long) * 8) - 4))))))
    abort ();

  if ((((c) << (shift1)) | ((c) >> ((sizeof (c) * 8) - (shift1)))) != (((((char)0xf234)) << (4)) | ((((char)0xf234)) >> ((sizeof (((char)0xf234)) * 8) - (4)))))
    abort ();

  if ((((c) << (4)) | ((c) >> ((sizeof (c) * 8) - (4)))) != (((((char)0xf234)) << (4)) | ((((char)0xf234)) >> ((sizeof (((char)0xf234)) * 8) - (4)))))
    abort ();

  if ((((s) << (shift1)) | ((s) >> ((sizeof (s) * 8) - (shift1)))) != (((((short)0xf234)) << (4)) | ((((short)0xf234)) >> ((sizeof (((short)0xf234)) * 8) - (4)))))
    abort ();

  if ((((s) << (4)) | ((s) >> ((sizeof (s) * 8) - (4)))) != (((((short)0xf234)) << (4)) | ((((short)0xf234)) >> ((sizeof (((short)0xf234)) * 8) - (4)))))
    abort ();

  if ((((i) << (shift1)) | ((i) >> ((sizeof (i) * 8) - (shift1)))) != (((((int)0xf234)) << (4)) | ((((int)0xf234)) >> ((sizeof (((int)0xf234)) * 8) - (4)))))
    abort ();

  if ((((i) << (4)) | ((i) >> ((sizeof (i) * 8) - (4)))) != (((((int)0xf234)) << (4)) | ((((int)0xf234)) >> ((sizeof (((int)0xf234)) * 8) - (4)))))
    abort ();

  if ((((l) << (shift1)) | ((l) >> ((sizeof (l) * 8) - (shift1)))) != (((((long)0xf2345678L)) << (4)) | ((((long)0xf2345678L)) >> ((sizeof (((long)0xf2345678L)) * 8) - (4)))))
    abort ();

  if ((((l) << (4)) | ((l) >> ((sizeof (l) * 8) - (4)))) != (((((long)0xf2345678L)) << (4)) | ((((long)0xf2345678L)) >> ((sizeof (((long)0xf2345678L)) * 8) - (4)))))
    abort ();

  if ((((ll) << (shift1)) | ((ll) >> ((sizeof (ll) * 8) - (shift1)))) != (((((long long)0xf2345678abcdef0LL)) << (4)) | ((((long long)0xf2345678abcdef0LL)) >> ((sizeof (((long long)0xf2345678abcdef0LL)) * 8) - (4)))))
    abort ();

  if ((((ll) << (4)) | ((ll) >> ((sizeof (ll) * 8) - (4)))) != (((((long long)0xf2345678abcdef0LL)) << (4)) | ((((long long)0xf2345678abcdef0LL)) >> ((sizeof (((long long)0xf2345678abcdef0LL)) * 8) - (4)))))
    abort ();

  if ((((ll) << (shift2)) | ((ll) >> ((sizeof (ll) * 8) - (shift2)))) != (((((long long)0xf2345678abcdef0LL)) << (((sizeof (long long) * 8) - 4))) | ((((long long)0xf2345678abcdef0LL)) >> ((sizeof (((long long)0xf2345678abcdef0LL)) * 8) - (((sizeof (long long) * 8) - 4))))))
    abort ();

  if ((((ll) << (((sizeof (long long) * 8) - 4))) | ((ll) >> ((sizeof (ll) * 8) - (((sizeof (long long) * 8) - 4))))) != (((((long long)0xf2345678abcdef0LL)) << (((sizeof (long long) * 8) - 4))) | ((((long long)0xf2345678abcdef0LL)) >> ((sizeof (((long long)0xf2345678abcdef0LL)) * 8) - (((sizeof (long long) * 8) - 4))))))
    abort ();

  exit (0);
}

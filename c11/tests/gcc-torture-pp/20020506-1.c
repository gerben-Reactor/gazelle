# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20020506-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20020506-1.c" 2







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
# 9 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20020506-1.c" 2

extern void abort (void);

void test1 (signed char c, int set);
void test2 (unsigned char c, int set);
void test3 (short s, int set);
void test4 (unsigned short s, int set);
void test5 (int i, int set);
void test6 (unsigned int i, int set);
void test7 (long long l, int set);
void test8 (unsigned long long l, int set);
# 32 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20020506-1.c"
void
test1 (signed char c, int set)
{
  if ((c & (127 +1)) == 0)
    {
      if (set) abort ();
    }
  else
    if (!set) abort ();

  if ((c & (127 +1)) != 0)
    {
      if (!set) abort ();
    }
  else
    if (set) abort ();

  if ((c & (127 +1)) == (127 +1))
    {
      if (!set) abort ();
    }
  else
    if (set) abort ();

  if ((c & (127 +1)) != (127 +1))
    {
      if (set) abort ();
    }
  else
    if (!set) abort ();
}

void
test2 (unsigned char c, int set)
{
  if ((c & (127 +1)) == 0)
    {
      if (set) abort ();
    }
  else
    if (!set) abort ();

  if ((c & (127 +1)) != 0)
    {
      if (!set) abort ();
    }
  else
    if (set) abort ();

  if ((c & (127 +1)) == (127 +1))
    {
      if (!set) abort ();
    }
  else
    if (set) abort ();

  if ((c & (127 +1)) != (127 +1))
    {
      if (set) abort ();
    }
  else
    if (!set) abort ();
}

void
test3 (short s, int set)
{
  if ((s & (32767 +1)) == 0)
    {
      if (set) abort ();
    }
  else
    if (!set) abort ();

  if ((s & (32767 +1)) != 0)
    {
      if (!set) abort ();
    }
  else
    if (set) abort ();

  if ((s & (32767 +1)) == (32767 +1))
    {
      if (!set) abort ();
    }
  else
    if (set) abort ();

  if ((s & (32767 +1)) != (32767 +1))
    {
      if (set) abort ();
    }
  else
    if (!set) abort ();
}

void
test4 (unsigned short s, int set)
{
  if ((s & (32767 +1)) == 0)
    {
      if (set) abort ();
    }
  else
    if (!set) abort ();

  if ((s & (32767 +1)) != 0)
    {
      if (!set) abort ();
    }
  else
    if (set) abort ();

  if ((s & (32767 +1)) == (32767 +1))
    {
      if (!set) abort ();
    }
  else
    if (set) abort ();

  if ((s & (32767 +1)) != (32767 +1))
    {
      if (set) abort ();
    }
  else
    if (!set) abort ();
}

void
test5 (int i, int set)
{
  if ((i & (2147483647 +1U)) == 0)
    {
      if (set) abort ();
    }
  else
    if (!set) abort ();

  if ((i & (2147483647 +1U)) != 0)
    {
      if (!set) abort ();
    }
  else
    if (set) abort ();

  if ((i & (2147483647 +1U)) == (2147483647 +1U))
    {
      if (!set) abort ();
    }
  else
    if (set) abort ();

  if ((i & (2147483647 +1U)) != (2147483647 +1U))
    {
      if (set) abort ();
    }
  else
    if (!set) abort ();
}

void
test6 (unsigned int i, int set)
{
  if ((i & (2147483647 +1U)) == 0)
    {
      if (set) abort ();
    }
  else
    if (!set) abort ();

  if ((i & (2147483647 +1U)) != 0)
    {
      if (!set) abort ();
    }
  else
    if (set) abort ();

  if ((i & (2147483647 +1U)) == (2147483647 +1U))
    {
      if (!set) abort ();
    }
  else
    if (set) abort ();

  if ((i & (2147483647 +1U)) != (2147483647 +1U))
    {
      if (set) abort ();
    }
  else
    if (!set) abort ();
}

void
test7 (long long l, int set)
{
  if ((l & (9223372036854775807LL +1ULL)) == 0)
    {
      if (set) abort ();
    }
  else
    if (!set) abort ();

  if ((l & (9223372036854775807LL +1ULL)) != 0)
    {
      if (!set) abort ();
    }
  else
    if (set) abort ();

  if ((l & (9223372036854775807LL +1ULL)) == (9223372036854775807LL +1ULL))
    {
      if (!set) abort ();
    }
  else
    if (set) abort ();

  if ((l & (9223372036854775807LL +1ULL)) != (9223372036854775807LL +1ULL))
    {
      if (set) abort ();
    }
  else
    if (!set) abort ();
}

void
test8 (unsigned long long l, int set)
{
  if ((l & (9223372036854775807LL +1ULL)) == 0)
    {
      if (set) abort ();
    }
  else
    if (!set) abort ();

  if ((l & (9223372036854775807LL +1ULL)) != 0)
    {
      if (!set) abort ();
    }
  else
    if (set) abort ();

  if ((l & (9223372036854775807LL +1ULL)) == (9223372036854775807LL +1ULL))
    {
      if (!set) abort ();
    }
  else
    if (set) abort ();

  if ((l & (9223372036854775807LL +1ULL)) != (9223372036854775807LL +1ULL))
    {
      if (set) abort ();
    }
  else
    if (!set) abort ();
}

int
main ()
{
  test1 (0, 0);
  test1 (127, 0);
  test1 ((-127 -1), 1);
  test1 ((127*2 +1), 1);

  test2 (0, 0);
  test2 (127, 0);
  test2 ((-127 -1), 1);
  test2 ((127*2 +1), 1);

  test3 (0, 0);
  test3 (32767, 0);
  test3 ((-32767 -1), 1);
  test3 ((32767 * 2 + 1), 1);

  test4 (0, 0);
  test4 (32767, 0);
  test4 ((-32767 -1), 1);
  test4 ((32767 * 2 + 1), 1);

  test5 (0, 0);
  test5 (2147483647, 0);
  test5 ((-2147483647 -1), 1);
  test5 ((2147483647 *2U +1U), 1);

  test6 (0, 0);
  test6 (2147483647, 0);
  test6 ((-2147483647 -1), 1);
  test6 ((2147483647 *2U +1U), 1);

  test7 (0, 0);
  test7 (9223372036854775807LL, 0);
  test7 ((-9223372036854775807LL -1), 1);
  test7 ((9223372036854775807LL * 2ULL + 1), 1);

  test8 (0, 0);
  test8 (9223372036854775807LL, 0);
  test8 ((-9223372036854775807LL -1), 1);
  test8 ((9223372036854775807LL * 2ULL + 1), 1);

  return 0;
}

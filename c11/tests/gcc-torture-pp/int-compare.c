# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/int-compare.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/int-compare.c" 2


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
# 4 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/int-compare.c" 2

gt (a, b)
{
  return a > b;
}

ge (a, b)
{
  return a >= b;
}

lt (a, b)
{
  return a < b;
}

le (a, b)
{
  return a <= b;
}

void
true (c)
{
  if (!c)
    abort();
}

void
false (c)
{
  if (c)
    abort();
}

f ()
{
  true (gt (2, 1));
  false (gt (1, 2));

  true (gt (2147483647, 0));
  false (gt (0, 2147483647));
  true (gt (2147483647, 1));
  false (gt (1, 2147483647));

  false (gt ((-2147483647 -1), 0));
  true (gt (0, (-2147483647 -1)));
  false (gt ((-2147483647 -1), 1));
  true (gt (1, (-2147483647 -1)));

  true (gt (2147483647, (-2147483647 -1)));
  false (gt ((-2147483647 -1), 2147483647));

  true (ge (2, 1));
  false (ge (1, 2));

  true (ge (2147483647, 0));
  false (ge (0, 2147483647));
  true (ge (2147483647, 1));
  false (ge (1, 2147483647));

  false (ge ((-2147483647 -1), 0));
  true (ge (0, (-2147483647 -1)));
  false (ge ((-2147483647 -1), 1));
  true (ge (1, (-2147483647 -1)));

  true (ge (2147483647, (-2147483647 -1)));
  false (ge ((-2147483647 -1), 2147483647));

  false (lt (2, 1));
  true (lt (1, 2));

  false (lt (2147483647, 0));
  true (lt (0, 2147483647));
  false (lt (2147483647, 1));
  true (lt (1, 2147483647));

  true (lt ((-2147483647 -1), 0));
  false (lt (0, (-2147483647 -1)));
  true (lt ((-2147483647 -1), 1));
  false (lt (1, (-2147483647 -1)));

  false (lt (2147483647, (-2147483647 -1)));
  true (lt ((-2147483647 -1), 2147483647));

  false (le (2, 1));
  true (le (1, 2));

  false (le (2147483647, 0));
  true (le (0, 2147483647));
  false (le (2147483647, 1));
  true (le (1, 2147483647));

  true (le ((-2147483647 -1), 0));
  false (le (0, (-2147483647 -1)));
  true (le ((-2147483647 -1), 1));
  false (le (1, (-2147483647 -1)));

  false (le (2147483647, (-2147483647 -1)));
  true (le ((-2147483647 -1), 2147483647));
}

main ()
{
  f ();
  exit (0);
}

# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr43385.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr43385.c" 2


extern void abort (void);

int e;

__attribute__((noinline)) void
foo (int x, int y)
{
  if (__builtin_expect (x, 0) && y != 0)
    e++;
}

__attribute__((noinline)) int
bar (int x, int y)
{
  if (__builtin_expect (x, 0) && y != 0)
    return 1;
  else
    return 0;
}

int
main (void)
{
  int z = 0;
  asm ("" : "+r" (z));
  foo (z + 2, z + 1);
  if (e != 1)
    abort ();
  foo (z + 2, z);
  if (e != 1)
    abort ();
  foo (z + 1, z + 1);
  if (e != 2)
    abort ();
  foo (z + 1, z);
  if (e != 2)
    abort ();
  foo (z, z + 1);
  if (e != 2)
    abort ();
  foo (z, z);
  if (e != 2)
    abort ();
  if (bar (z + 2, z + 1) != 1)
    abort ();
  if (bar (z + 2, z) != 0)
    abort ();
  if (bar (z + 1, z + 1) != 1)
    abort ();
  if (bar (z + 1, z) != 0)
    abort ();
  if (bar (z, z + 1) != 0)
    abort ();
  if (bar (z, z) != 0)
    abort ();
  return 0;
}

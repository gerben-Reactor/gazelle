# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr27260.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr27260.c" 2


extern void abort (void);
extern void *memset (void *, int, long unsigned int);

char buf[65];

void
foo (int x)
{
  memset (buf, x != 2 ? 1 : 0, 64);
}

int
main (void)
{
  int i;
  buf[64] = 2;
  for (i = 0; i < 64; i++)
    if (buf[i] != 0)
      abort ();
  foo (0);
  for (i = 0; i < 64; i++)
    if (buf[i] != 1)
      abort ();
  foo (2);
  for (i = 0; i < 64; i++)
    if (buf[i] != 0)
      abort ();
  if (buf[64] != 2)
    abort ();
  return 0;
}

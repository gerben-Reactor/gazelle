# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/loop-ivopts-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/loop-ivopts-2.c" 2
# 22 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/loop-ivopts-2.c"
extern void abort (void);

void
check (unsigned int *l)
{
  int i;
  for (i = 0; i < 288; i++)
    if (l[i] != 7 + (i < 256 || i >= 280) + (i >= 144 && i < 256))
      abort ();
}

int
main (void)
{
  int i;
  unsigned int l[288];

  for (i = 0; i < 144; i++)
    l[i] = 8;
  for (; i < 256; i++)
    l[i] = 9;
  for (; i < 280; i++)
    l[i] = 7;
  for (; i < 288; i++)
    l[i] = 8;
  check (l);
  return 0;
}

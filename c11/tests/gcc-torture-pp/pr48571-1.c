# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr48571-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr48571-1.c" 2


unsigned int c[624];
void __attribute__((noinline))
bar (void)
{
  unsigned int i;

  for (i = 1; i < 624; ++i)
    *(unsigned int *)((void *)c + (long unsigned int)i * (sizeof (int)))
 = 2 * *(unsigned int *)((void *)c + ((long unsigned int)i +
          ((long unsigned int)-(sizeof (int)))/(sizeof (int))) * (sizeof (int)));
}
extern void abort (void);
int
main()
{
  unsigned int i, j;
  for (i = 0; i < 624; ++i)
    c[i] = 1;
  bar();
  j = 1;
  for (i = 0; i < 624; ++i)
    {
      if (c[i] != j)
 abort ();
      j = j * 2;
    }
  return 0;
}

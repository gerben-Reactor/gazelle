# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20140425-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20140425-1.c" 2



extern void abort (void);

static void __attribute__((noinline))
set (unsigned long *l)
{
  *l = 31;
}

int main (void)
{
  unsigned long l;
  int i;

  set (&l);
  i = (int) l;
  l = (unsigned long)(2U << i);
  if (l != 0)
    abort ();
  return 0;
}

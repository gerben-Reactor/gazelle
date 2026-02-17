# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20180131-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20180131-1.c" 2



extern void abort (void);

typedef union
{
  signed short ss;
  unsigned short us;
  int x;
} U;

int f(int x, int y, int z, int a, U u) __attribute__((noclone, noinline));

int f(int x, int y, int z, int a, U u)
{
  return (u.ss <= 0) + u.us;
}

int main (void)
{
  U u = { .ss = -1 };

  if (f (0, 0, 0, 0, u) != (1 << sizeof (short) * 8))
    abort ();

  return 0;
}

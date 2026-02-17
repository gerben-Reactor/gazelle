# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr64718.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr64718.c" 2
static int __attribute__ ((noinline, noclone))
swap (int x)
{
  return (unsigned short) ((unsigned short) x << 8 | (unsigned short) x >> 8);
}

static int a = 0x1234;

int
main (void)
{
  int b = 0x1234;
  if (swap (a) != 0x3412)
    __builtin_abort ();
  if (swap (b) != 0x3412)
    __builtin_abort ();
  return 0;
}

# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/alias-3.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/alias-3.c" 2


static int a=0;
extern int b __attribute__ ((alias("a")));
__attribute__ ((noinline))
static void inc(void)
{
  b++;
}
int
main()
{
  a=0;
  inc ();
  if (a!=1)
    __builtin_abort ();
  return 0;
}

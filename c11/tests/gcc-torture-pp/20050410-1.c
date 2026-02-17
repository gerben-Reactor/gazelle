# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20050410-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20050410-1.c" 2
void abort (void);
void exit (int);

int s = 200;
int __attribute__((noinline))
foo (void)
{
  return (signed char) (s - 100) - 5;
}
int
main (void)
{
  if (foo () != 95)
    abort ();
  exit (0);
}

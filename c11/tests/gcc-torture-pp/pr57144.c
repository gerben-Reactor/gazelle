# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr57144.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr57144.c" 2
extern void abort (void);
extern void exit (int);

void __attribute__ ((noinline))
foo(int a)
{
  int z = a > 0 ? a : -a;
  long long x = z;
  if (x > 0x100000000LL)
    abort ();
  else
    exit (0);
}

int
main()
{
  foo (1);
}

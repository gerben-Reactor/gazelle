# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/struct-ini-3.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/struct-ini-3.c" 2
void abort (void);
void exit (int);

struct
{
  unsigned int f1:1, f2:1, f3:3, f4:3, f5:2, f6:1, f7:1;
} result = {1, 1, 7, 7, 3, 1, 1};

int
main (void)
{
  if ((result.f3 & ~7) != 0 || (result.f4 & ~7) != 0)
    abort ();
  exit (0);
}

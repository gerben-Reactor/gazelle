# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/struct-ini-4.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/struct-ini-4.c" 2
void abort (void);
void exit (int);

struct s {
  int a[3];
  int c[3];
};

struct s s = {
  c: {1, 2, 3}
};

int
main(void)
{
  if (s.c[0] != 1)
    abort ();
  exit (0);
}

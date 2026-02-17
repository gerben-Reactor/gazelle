# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/struct-ini-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/struct-ini-1.c" 2
void abort (void);
void exit (int);

struct S
{
  char f1;
  int f2[2];
};

struct S object = {'X', 8, 9};

int
main (void)
{
  if (object.f1 != 'X' || object.f2[0] != 8 || object.f2[1] != 9)
    abort ();
  exit (0);
}

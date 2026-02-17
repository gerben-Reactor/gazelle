# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/struct-ini-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/struct-ini-2.c" 2
void abort (void);
void exit (int);

struct {
  int a:4;
  int :4;
  int b:4;
  int c:4;
} x = { 2,3,4 };

int
main (void)
{
  if (x.a != 2)
    abort ();
  if (x.b != 3)
    abort ();
  if (x.c != 4)
    abort ();
  exit (0);
}

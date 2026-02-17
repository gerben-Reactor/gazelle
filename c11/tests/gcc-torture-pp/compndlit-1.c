# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/compndlit-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/compndlit-1.c" 2


void abort (void);
void exit (int);
# 13 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/compndlit-1.c"
struct S
{
  int a:3;
  unsigned b:1, c:28;
};

struct S x = {1, 1, 1};

int
main (void)
{
  x = (struct S) {b:0, a:0, c:({ struct S o = x; o.a == 1 ? 10 : 20;})};
  if (x.c != 10)
    abort ();
  exit (0);
}

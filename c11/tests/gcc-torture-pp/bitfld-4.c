# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/bitfld-4.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/bitfld-4.c" 2
# 11 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/bitfld-4.c"
extern void abort (void);

struct s { int a:12, b:20; };

struct s x = { -123, -456 };

int
main (void)
{
  if (x.a != -123U || x.b != -456U)
    abort ();
  return 0;
}

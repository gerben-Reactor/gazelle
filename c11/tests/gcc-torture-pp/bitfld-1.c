# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/bitfld-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/bitfld-1.c" 2
# 17 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/bitfld-1.c"
extern void abort ();

int
main(int argc, char *argv[])
{
  struct x { signed int i : 7; unsigned int u : 7; } bit;

  unsigned int u;
  int i;
  unsigned int unsigned_result = -13U % 61;
  int signed_result = -13 % 61;

  bit.u = 61, u = 61;
  bit.i = -13, i = -13;

  if (i % u != unsigned_result)
    abort ();
  if (i % (unsigned int) u != unsigned_result)
    abort ();



  if (i % bit.u != signed_result)
    abort ();

  if (bit.i % bit.u != signed_result)
    abort ();



  if (i % (unsigned int) bit.u != unsigned_result)
    abort ();

  if (bit.i % (unsigned int) bit.u != unsigned_result)
    abort ();

  return 0;
}

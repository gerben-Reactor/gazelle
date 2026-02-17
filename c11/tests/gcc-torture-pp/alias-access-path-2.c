# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/alias-access-path-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/alias-access-path-2.c" 2
int
main (int argc, char **argv)
{
  int c;
  unsigned char out[][1] = { {71}, {71}, {71} };

  for (int i = 0; i < 3; i++)
    if (!out[i][0])
      __builtin_abort ();
  return 0;
}

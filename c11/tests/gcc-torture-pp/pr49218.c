# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr49218.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr49218.c" 2

typedef __int128 L;



float f;

int
main ()
{
  L i = f;
  if (i <= 10)
    do
      {
 ++i;
 asm ("");
      }
    while (i != 11);
  return 0;
}

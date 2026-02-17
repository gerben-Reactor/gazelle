# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20051104-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20051104-1.c" 2


struct
{
  int len;
  char *name;
} s;

int
main (void)
{
  s.len = 0;
  s.name = "";
  if (s.name [s.len] != 0)
    s.name [s.len] = 0;
  return 0;
}

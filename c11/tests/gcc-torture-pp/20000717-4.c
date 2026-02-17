# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20000717-4.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20000717-4.c" 2



struct
{
  int offset;
  struct slot
  {
    int field[6];
  }
  slot[4];
} s;

int
x ()
{
  int toggle = 0;
  int r = s.slot[0].field[!toggle];
  return r;
}

int
main ()
{
  return 0;
}

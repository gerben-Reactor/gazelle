# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20230630-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20230630-1.c" 2
struct S {
  short int i : 12;
  char c1 : 1;
  char c2 : 1;
  char c3 : 1;
  char c4 : 1;
};

int main (void)
{
  struct S s0 = { 341, 1, 1, 1, 1 };
  char *p = (char *) &s0;


  if (*p != 85)
    __builtin_abort ();





  return 0;
}

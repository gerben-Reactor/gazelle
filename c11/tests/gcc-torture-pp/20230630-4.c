# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20230630-4.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20230630-4.c" 2






struct S {
  int i : 24;
  char c1 : 1;
  char c2 : 1;
  char c3 : 1;
  char c4 : 1;
  char c5 : 1;
  char c6 : 1;
  char c7 : 1;
  char c8 : 1;
} __attribute__((scalar_storage_order("big-endian")));;

int main (void)
{
  struct S s0 = { 1193046, 1, 1, 1, 1, 1, 1, 1, 1 };
  char *p = (char *) &s0;


  if (*p != 18)
    __builtin_abort ();





  return 0;
}

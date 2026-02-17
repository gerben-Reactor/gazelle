# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr80692.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr80692.c" 2



int main () {
 _Decimal64 d64 = -0.DD;

 if (d64 != 0.DD)
  __builtin_abort ();

 if (d64 != -0.DD)
  __builtin_abort ();

 return 0;
}

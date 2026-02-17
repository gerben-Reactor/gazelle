# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/bitfld-signed1-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/bitfld-signed1-1.c" 2




struct {
  signed a : 1;
} b = {-1};
char c;
int main()
{
  if ((b.a ^ 1UL) < 3)
    __builtin_abort();
}

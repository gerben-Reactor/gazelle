# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr42570.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr42570.c" 2
typedef unsigned char uint8_t;
uint8_t foo[1][0];
extern void abort (void);
int main()
{
  if (sizeof (foo) != 0)
    abort ();
  return 0;
}

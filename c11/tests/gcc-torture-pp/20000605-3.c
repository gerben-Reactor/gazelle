# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20000605-3.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20000605-3.c" 2
void abort (void);

struct F { int x; int y; };

int main()
{
  int timeout = 0;
  int x = 0;
  while (1)
    {
      const struct F i = { x++, };
      if (i.x > 0)
 break;
      if (++timeout > 5)
 goto die;
    }
  return 0;
 die:
  abort ();
}

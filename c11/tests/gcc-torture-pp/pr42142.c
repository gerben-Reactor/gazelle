# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr42142.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr42142.c" 2
int __attribute__((noinline,noclone))
sort(int L)
{
  int end[2] = { 10, 10, }, i=0, R;
  while (i<2)
    {
      R = end[i];
      if (L<R)
        {
          end[i+1] = 1;
          end[i] = 10;
          ++i;
        }
      else
        break;
    }
  return i;
}
extern void abort (void);
int main()
{
  if (sort (5) != 1)
    abort ();
  return 0;
}

# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/unroll-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/unroll-1.c" 2


extern void abort (void);
extern void exit (int);

inline int
f (int x)
{
  return (x + 1);
}

int
main (void)
{
  int a = 0 ;

  while ( (f(f(f(f(f(f(f(f(f(f(1))))))))))) + a < 12 )
    {
      a++;
      exit (0);
    }
  if (a != 1)
    abort();
}

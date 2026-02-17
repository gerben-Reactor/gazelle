# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr58209.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr58209.c" 2


extern void abort (void);
typedef long int T;
T buf[1024];

T *
foo (T n)
{
  if (n == 0)
    return (T *) buf;
  T s = (T) foo (n - 1);
  return (T *) (s + sizeof (T));
}

T *
bar (T n)
{
  if (n == 0)
    return buf;
  return foo (n - 1) + 1;
}

int
main ()
{
  int i;
  for (i = 0; i < 27; i++)
    if (foo (i) != buf + i || bar (i) != buf + i)
      abort ();
  return 0;
}

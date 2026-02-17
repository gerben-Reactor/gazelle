# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20001228-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20001228-1.c" 2
void abort (void);
void exit (int);

int foo1(void)
{
  union {
    char a[sizeof (unsigned)];
    unsigned b;
  } u;

  u.b = 0x01;
  return u.a[0];
}

int foo2(void)
{
  volatile union {
    char a[sizeof (unsigned)];
    unsigned b;
  } u;

  u.b = 0x01;
  return u.a[0];
}

int main(void)
{
  if (foo1() != foo2())
    abort ();
  exit (0);
}

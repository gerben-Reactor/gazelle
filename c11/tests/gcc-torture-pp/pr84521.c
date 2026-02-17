# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr84521.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr84521.c" 2



extern void abort (void);

void
broken_longjmp (void *p)
{
  __builtin_longjmp (p, 1);
}

volatile int x = 256;
void *volatile p = (void*)&x;
void *volatile p1;

void
test (void)
{
  void *buf[5];
  void *volatile q = p;

  if (!__builtin_setjmp (buf))
    broken_longjmp (buf);


  if (p != q)
    abort ();
}

void
test2 (void)
{
  void *volatile q = p;
  p1 = __builtin_alloca (x);
  test ();


  if (p != q)
    abort ();
}

int
main (void)
{
  void *volatile q = p;
  test ();
  test2 ();

  if (p != q)
    abort ();

  return 0;
}

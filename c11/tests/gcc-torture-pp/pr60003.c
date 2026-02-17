# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr60003.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr60003.c" 2



extern void abort (void);

unsigned long long jmp_buf[5];

__attribute__((noinline, noclone)) void
baz (void)
{
  __builtin_longjmp (&jmp_buf, 1);
}

void
bar (void)
{
  baz ();
}

__attribute__((noinline, noclone)) int
foo (int x)
{
  int a = 0;

  if (__builtin_setjmp (&jmp_buf) == 0)
    {
      while (1)
 {
   a = 1;
   bar ();
 }
    }
  else
    {
      if (a == 0)
 return 0;
      else
 return x;
    }
}

int
main ()
{
  if (foo (1) == 0)
    abort ();

  return 0;
}

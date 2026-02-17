# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr64242.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr64242.c" 2


extern void abort (void);

__attribute ((noinline)) void
broken_longjmp (void *p)
{
  void *buf[32];
  __builtin_memcpy (buf, p, 5 * sizeof (void*));
  __builtin_memset (p, 0, 5 * sizeof (void*));

  __builtin_longjmp (buf, 1);
}

volatile int x = 0;
char *volatile p;
char *volatile q;

int
main ()
{
  void *buf[5];
  p = __builtin_alloca (x);
  q = __builtin_alloca (x);
  if (!__builtin_setjmp (buf))
    broken_longjmp (buf);



  p = q + (q - p);


  if (p != __builtin_alloca (x))
    abort ();

  return 0;
}

# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/built-in-setjmp.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/built-in-setjmp.c" 2


extern int strcmp(const char *, const char *);
extern char *strcpy(char *, const char *);
extern void abort(void);
extern void exit(int);

void *buf[20];

void __attribute__((noinline))
sub2 (void)
{
  __builtin_longjmp (buf, 1);
}

int
main ()
{
  char *p = (char *) __builtin_alloca (20);

  strcpy (p, "test");

  if (__builtin_setjmp (buf))
    {
      if (strcmp (p, "test") != 0)
 abort ();

      exit (0);
    }

  {
    int *q = (int *) __builtin_alloca (p[2] * sizeof (int));
    int i;

    for (i = 0; i < p[2]; i++)
      q[i] = 0;

    while (1)
      sub2 ();
  }
}

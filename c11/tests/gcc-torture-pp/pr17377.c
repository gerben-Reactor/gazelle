# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr17377.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr17377.c" 2




void abort (void);
void exit (int);

int calls = 0;

void *f (int) __attribute__ ((__noinline__));
void *
f (int i)
{




  if (calls++ == 0)
    return __builtin_return_address (0);

  switch (i)
    {
    case 1:
      return f;
    case 0:
      return __builtin_return_address (0);
    }
  return 0;
}

int x;

void *y (int i) __attribute__ ((__noinline__,__noclone__));
void *
y (int i)
{
  x = 0;






  return (char *) f (i) + x++;
}

int
main (void)
{
  void *v = y (4);
  if (y (1) != f





      || y (0) != v
      || y (3) != 0
      || y (-1) != 0
      || calls != 5)
    abort ();
  exit (0);
}

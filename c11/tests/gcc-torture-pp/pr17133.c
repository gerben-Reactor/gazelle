# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr17133.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr17133.c" 2
extern void abort (void);

int foo = 0;
void *bar = 0;
unsigned int baz = 100;

void *pure_alloc ()
{
  void *res;

  while (1)
    {
      res = (void *) ((((unsigned int) (foo + bar))) & ~1);
      foo += 2;
      if (foo < baz)
        return res;
      foo = 0;
    }
}

int main ()
{
  pure_alloc ();
  if (!foo)
    abort ();
  return 0;
}

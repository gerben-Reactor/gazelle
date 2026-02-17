# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr64756.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr64756.c" 2


int a, *tmp, **c = &tmp;
volatile int d;
static int *volatile *e = &tmp;
unsigned int f;

static void
fn1 (int *p)
{
  int g;
  for (; f < 1; f++)
    for (g = 1; g >= 0; g--)
      {
 d || d;
 *c = p;

 if (tmp != &a)
   __builtin_abort ();

 *e = 0;
      }
}

int
main ()
{
  fn1 (&a);
  return 0;
}

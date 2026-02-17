# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr58365.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr58365.c" 2


extern void abort (void);

struct S
{
  volatile int a;
  int b, c, d, e;
} f;
static struct S g, h;
int i = 1;

char
foo (void)
{
  return i;
}

static struct S
bar (void)
{
  if (foo ())
    return f;
  return g;
}

int
main ()
{
  h = bar ();
  f.b = 1;
  if (h.b != 0)
    abort ();
  return 0;
}

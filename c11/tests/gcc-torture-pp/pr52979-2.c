# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr52979-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr52979-2.c" 2



extern void abort (void);
int c, d, e;

void
foo (void)
{
}

struct __attribute__((packed)) S { int g : 31; int h : 6; };
static struct S b = { 1 };
struct S a = { 1 };

void
bar (void)
{
  a.h = 1;
  struct S f = { };
  b = f;
  e = 0;
  if (d)
    c = a.g;
}

void
baz (void)
{
  bar ();
  a = b;
}

int
main ()
{
  baz ();
  if (a.g)
    abort ();
  return 0;
}

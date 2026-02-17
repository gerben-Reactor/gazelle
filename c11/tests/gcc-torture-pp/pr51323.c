# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr51323.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr51323.c" 2


extern void abort (void);
struct S { int a, b, c; };
int v;

__attribute__((noinline, noclone)) void
foo (int x, int y, int z)
{
  if (x != v || y != 0 || z != 9)
    abort ();
}

static inline int
baz (const struct S *p)
{
  return p->b;
}

__attribute__((noinline, noclone)) void
bar (int x, struct S y)
{
  foo (baz (&y), 0, x);
}

int
main ()
{
  struct S s;
  v = 3; s.a = v - 1; s.b = v; s.c = v + 1;
  bar (9, s);
  v = 17; s.a = v - 1; s.b = v; s.c = v + 1;
  bar (9, s);
  return 0;
}

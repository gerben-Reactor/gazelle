# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20070919-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20070919-1.c" 2


typedef long unsigned int size_t;
int memcmp (const void *, const void *, size_t);
void abort (void);

void
__attribute__((noinline))
bar (void *x, void *y)
{
  struct S { char w[8]; } *p = x, *q = y;
  if (memcmp (p->w, "zyxwvut", 8) != 0)
    abort ();
  if (memcmp (q[0].w, "abcdefg", 8) != 0)
    abort ();
  if (memcmp (q[1].w, "ABCDEFG", 8) != 0)
    abort ();
  if (memcmp (q[2].w, "zyxwvut", 8) != 0)
    abort ();
  if (memcmp (q[3].w, "zyxwvut", 8) != 0)
    abort ();
}

void
__attribute__((noinline))
foo (void *x, int y)
{
  struct S { char w[y]; } *p = x, a;
  int i;
  a = ({ struct S b; b = p[2]; p[3] = b; });
  bar (&a, x);
}

int
main (void)
{
  struct S { char w[8]; } p[4]
    = { "abcdefg", "ABCDEFG", "zyxwvut", "ZYXWVUT" };
  foo (p, 8);
  return 0;
}

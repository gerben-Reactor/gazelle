# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr44164.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr44164.c" 2
struct X {
    struct Y {
 struct YY {
     struct Z {
  int i;
     } c;
 } bb;
    } b;
} a;
int __attribute__((noinline, noclone))
foo (struct Z *p)
{
  int i = p->i;
  a.b = (struct Y){};
  return p->i + i;
}
extern void abort (void);
int main()
{
  a.b.bb.c.i = 1;
  if (foo (&a.b.bb.c) != 1)
    abort ();
  return 0;
}

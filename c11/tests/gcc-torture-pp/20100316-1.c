# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20100316-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20100316-1.c" 2
struct Foo {
  int i;
  unsigned precision : 10;
  unsigned blah : 3;
} f;

int __attribute__((noinline,noclone))
foo (struct Foo *p)
{
  struct Foo *q = p;
  return (*q).precision;
}

extern void abort (void);

int main()
{
  f.i = -1;
  f.precision = 0;
  f.blah = -1;
  if (foo (&f) != 0)
    abort ();
  return 0;
}

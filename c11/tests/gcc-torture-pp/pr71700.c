# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr71700.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr71700.c" 2
struct S
{
  signed f0 : 16;
  unsigned f1 : 1;
};

int b;
static struct S c[] = {{-1, 0}, {-1, 0}};
struct S d;

int
main ()
{
  struct S e = c[0];
  d = e;
  if (d.f1 != 0)
    __builtin_abort ();
  return 0;
}

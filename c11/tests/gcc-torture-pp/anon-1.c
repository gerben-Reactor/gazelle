# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/anon-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/anon-1.c" 2





void abort (void);

struct
{
  int x;
  struct
  {
    int a;
    union
    {
      int b;
    };
  };
} foo;

int
main(int argc, char *argv[])
{
  foo.b = 6;
  foo.a = 5;

  if (foo.b != 6)
    abort ();

  return 0;
}

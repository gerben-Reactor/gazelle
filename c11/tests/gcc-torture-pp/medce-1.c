# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/medce-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/medce-1.c" 2

extern void abort (void);
extern void link_error (void);

static int ok = 0;

void bar (void)
{
  ok = 1;
}

void foo(int x)
{
  switch (x)
  {
  case 0:
    if (0)
    {
      link_error();
  case 1:
      bar();
    }
  }
}

int main()
{
  foo (1);
  if (!ok)
    abort ();
  return 0;
}

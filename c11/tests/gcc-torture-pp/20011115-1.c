# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20011115-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20011115-1.c" 2
extern void exit (int);

static inline int
foo (void)
{




  return 0;

}

static inline int
bar (void)
{
  if (foo == foo)
    return 1;
  else
    return foo ();
}

int main (void)
{
  exit (0);
}

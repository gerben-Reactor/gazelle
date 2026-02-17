# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20020107-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20020107-1.c" 2




extern void abort (void);
extern void exit (int);

static void
bar (int x)
{
  if (!x)
    abort ();
}

char buf[10];

inline char *
foo (char *tmp)
{
  asm ("" : "=r" (tmp) : "0" (tmp));
  return tmp + 2;
}

int
main (void)
{
  bar ((foo (buf) - 1 - buf) == 1);
  exit (0);
}

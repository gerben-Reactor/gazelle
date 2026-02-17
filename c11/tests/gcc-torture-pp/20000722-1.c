# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20000722-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20000722-1.c" 2
void abort (void);
void exit (int);

struct s { char *p; int t; };

extern void bar (void);
extern void foo (struct s *);

int main(void)
{
  bar ();
  bar ();
  exit (0);
}

void
bar (void)
{
  foo (& (struct s) { "hi", 1 });
}

void foo (struct s *p)
{
  if (p->t != 1)
    abort();
  p->t = 2;
}

# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20020227-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20020227-1.c" 2




void abort (void);
void exit (int);

typedef __complex__ float cf;
struct x { char c; cf f; } __attribute__ ((__packed__));
extern void f2 (struct x*);
extern void f1 (void);
int
main (void)
{
  f1 ();
  exit (0);
}

void
f1 (void)
{
  struct x s;
  s.f = 1;
  s.c = 42;
  f2 (&s);
}

void
f2 (struct x *y)
{
  if (y->f != 1 || y->c != 42)
    abort ();
}

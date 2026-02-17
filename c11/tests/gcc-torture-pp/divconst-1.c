# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/divconst-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/divconst-1.c" 2
void abort (void);
void exit (int);

typedef struct
{
  unsigned a, b, c, d;
} t1;

void
f (t1 *ps)
{
    ps->a = 10000;
    ps->b = ps->a / 3;
    ps->c = 10000;
    ps->d = ps->c / 3;
}

int
main (void)
{
  t1 s;
  f (&s);
  if (s.a != 10000 || s.b != 3333 || s.c != 10000 || s.d != 3333)
    abort ();
  exit (0);
}

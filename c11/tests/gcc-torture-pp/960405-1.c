# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/960405-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/960405-1.c" 2
void abort (void);
void exit (int);



long double x = 5.9486574767861588254287966331400356538172e4931L + 5.9486574767861588254287966331400356538172e4931L;
long double y = 2.0L * 5.9486574767861588254287966331400356538172e4931L;

int
main (void)
{

  if (x != y)
    abort ();

  exit (0);
}

# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/bf-layout-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/bf-layout-1.c" 2
void abort (void);
void exit (int);

struct { long f8:8; long f24:24; } a;
struct { long f32:32; } b;

int
main (void)
{
  if (sizeof (a) != sizeof (b))
    abort ();
  exit (0);
}

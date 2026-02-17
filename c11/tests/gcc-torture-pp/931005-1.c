# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/931005-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/931005-1.c" 2
void abort (void);
void exit (int);

typedef struct
{
  char x;
} T;

T
f (s1)
     T s1;
{
  T s1a;
  s1a.x = s1.x;
  return s1a;
}

int
main (void)
{
  T s1a, s1b;
  s1a.x = 100;
  s1b = f (s1a);
  if (s1b.x != 100)
    abort ();
  exit (0);
}

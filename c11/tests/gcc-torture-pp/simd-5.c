# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/simd-5.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/simd-5.c" 2


void abort (void);

typedef short Q __attribute__((vector_size(8)));

Q q1 = {1, 2}, q2 = {3, 4}, q3 = {5, 6}, q4 = {7, 8};

Q w1, w2, w3, w4;
Q z1, z2, z3, z4;

volatile int dummy;

void __attribute__((__noinline__))
func0 (void)
{
  dummy = 1;
}

void __attribute__((__noinline__))
func1 (void)
{
  Q a, b;
  a = q1 * q2;
  b = q3 * q4;
  w1 = a;
  w2 = b;
  func0 ();
  w3 = a;
  w4 = b;
}

void __attribute__((__noinline__))
func2 (void)
{
  Q a, b;
  a = q1 + q2;
  b = q3 - q4;
  z1 = a;
  z2 = b;
  func1 ();
  z3 = a;
  z4 = b;
}

int
main (void)
{
  func2 ();

  if (__builtin_memcmp (&w1, &w3, sizeof (Q)) != 0)
    abort ();
  if (__builtin_memcmp (&w2, &w4, sizeof (Q)) != 0)
    abort ();
  if (__builtin_memcmp (&z1, &z3, sizeof (Q)) != 0)
    abort ();
  if (__builtin_memcmp (&z2, &z4, sizeof (Q)) != 0)
    abort ();

  return 0;
}

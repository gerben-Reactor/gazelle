# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr92618.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr92618.c" 2


typedef long long __m128i __attribute__((__vector_size__(2 * sizeof (long long)),__may_alias__));

double a[4];
unsigned long long b[4];

__attribute__((noipa)) __m128i
bar (void)
{
  static int cnt;
  cnt += 2;
  return (__m128i) { cnt, cnt + 1 };
}


typedef double __m128d __attribute__((__vector_size__(2 * sizeof (double)),__may_alias__));

__attribute__((noipa)) __m128i
qux (void)
{
  static double cnt;
  cnt += 2.0;
  return (__m128i) (__m128d) { cnt, cnt + 1.0 };
}


void
foo (unsigned long long *x)
{
  __m128i c = bar ();
  __m128i d = bar ();
  *(__m128i *) &b[0] = c;
  *(__m128i *) &b[2] = d;
  *x = b[0] + b[1] + b[2] + b[3];
}

void
baz (double *x)
{

  __m128i c = qux ();
  __m128i d = qux ();
  *(__m128i *) &a[0] = c;
  *(__m128i *) &a[2] = d;
  *x = a[0] + a[1] + a[2] + a[3];

}

int
main ()
{
  unsigned long long c = 0;
  foo (&c);
  if (c != 2 + 3 + 4 + 5)
    __builtin_abort ();

  double d = 0.0;
  baz (&d);
  if (d != 2.0 + 3.0 + 4.0 + 5.0)
    __builtin_abort ();

}

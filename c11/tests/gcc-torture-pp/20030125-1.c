# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20030125-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20030125-1.c" 2



void abort(void);
double sin(double);
double floor(double);
float
t(float a)
{
 return sin(a);
}
float
q(float a)
{
 return floor(a);
}
double
q1(float a)
{
 return floor(a);
}
int
main(void)
{
# 33 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20030125-1.c"
 return 0;
}
__attribute__ ((weak))
double
floor(double a)
{
 abort ();
}
__attribute__ ((weak))
float
floorf(float a)
{
 return a;
}
__attribute__ ((weak))
double
sin(double a)
{
 return a;
}
__attribute__ ((weak))
float
sinf(float a)
{
 abort ();
}

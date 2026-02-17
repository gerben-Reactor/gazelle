# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr43008.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr43008.c" 2
int i;
struct X {
  int *p;
};
struct X * __attribute__((malloc))
my_alloc (void)
{
  struct X *p = __builtin_malloc (sizeof (struct X));
  p->p = &i;
  return p;
}
extern void abort (void);
int main()
{
  struct X *p, *q;
  p = my_alloc ();
  q = my_alloc ();
  *(p->p) = 1;
  *(q->p) = 0;
  if (*(p->p) != 0)
    abort ();
  return 0;
}

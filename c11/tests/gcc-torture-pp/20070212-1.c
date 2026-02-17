# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20070212-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20070212-1.c" 2
struct f
{
  int i;
};

int g(int i, int c, struct f *ff, int *p)
{
  int *t;
  if (c)
   t = &i;
  else
   t = &ff->i;
  *p = 0;
  return *t;
}

extern void abort(void);

int main()
{
  struct f f;
  f.i = 1;
  if (g(5, 0, &f, &f.i) != 0)
    abort ();
  return 0;
}

# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/990208-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/990208-1.c" 2






void abort (void);
void exit (int);

static void *ptr1, *ptr2;
static int i = 1;

static __inline__ void doit(void **pptr, int cond)
{
  if (cond) {
  here:
    *pptr = &&here;
  }
}

__attribute__ ((noinline))
static void f(int cond)
{
  doit (&ptr1, cond);
}

__attribute__ ((noinline))
static void g(int cond)
{
  doit (&ptr2, cond);
}

__attribute__ ((noinline))
static void bar(void);

int main()
{
  f (i);
  bar();
  g (i);






  exit (0);
}

void bar(void) { }

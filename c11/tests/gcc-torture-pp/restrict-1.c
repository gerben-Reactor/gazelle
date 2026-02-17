# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/restrict-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/restrict-1.c" 2






extern void abort ();

typedef struct
{
  int i, dummy;
} A;

inline A foo (const A* p, const A* q)
{
  return (A){p->i+q->i};
}

void bar (A* __restrict__ p)
{
  *p=foo(p,p);
  if (p->i!=2)
    abort();
}

int main ()
{
  A a={1};
  bar(&a);
  return 0;
}

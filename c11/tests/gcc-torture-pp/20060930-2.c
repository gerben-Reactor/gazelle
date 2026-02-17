# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20060930-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20060930-2.c" 2


extern void abort (void);

struct S { struct S *s; } s;
struct T { struct T *t; } t;

static inline void
foo (void *s)
{
  struct T *p = s;
  __builtin_memcpy (&p->t, &t.t, sizeof (t.t));
}

void *
__attribute__((noinline))
bar (void *p, struct S *q)
{
  q->s = &s;
  foo (p);
  return q->s;
}

int
main (void)
{
  t.t = &t;
  if (bar (&s, &s) != (void *) &t)
    abort ();
  return 0;
}

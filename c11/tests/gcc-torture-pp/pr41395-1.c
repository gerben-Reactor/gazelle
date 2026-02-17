# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr41395-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr41395-1.c" 2
struct VEC_char_base
{
  unsigned num;
  unsigned alloc;
  short vec[1];
};

short __attribute__((noinline))
foo (struct VEC_char_base *p, int i)
{
  short *q;
  p->vec[i] = 0;
  q = &p->vec[8];
  *q = 1;
  return p->vec[i];
}

extern void abort (void);
extern void *malloc (long unsigned int);

int
main()
{
  struct VEC_char_base *p = malloc (sizeof (struct VEC_char_base) + 256);
  if (foo (p, 8) != 1)
    abort ();
  return 0;
}

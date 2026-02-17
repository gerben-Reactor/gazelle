# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20120105-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20120105-1.c" 2
struct __attribute__((packed)) S
{
  int a, b, c;
};

static int __attribute__ ((noinline,noclone))
extract(const char *p)
{
  struct S s;
  __builtin_memcpy (&s, p, sizeof(struct S));
  return s.a;
}

volatile int i;

int main (void)
{
  char p[sizeof(struct S) + 1];

  __builtin_memset (p, 0, sizeof(struct S) + 1);
  i = extract (p + 1);

  return 0;
}

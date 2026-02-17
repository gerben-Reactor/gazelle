# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/alloca-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/alloca-1.c" 2




typedef long unsigned int size_t;
void abort (void);

struct dummy { int x __attribute__((aligned)); };


_Bool foo(void)
{
  char *p = __builtin_alloca(32);
  return ((size_t)p & (__alignof__(struct dummy) - 1)) == 0;
}

int main()
{
  if (!foo())
    abort ();
  return 0;
}

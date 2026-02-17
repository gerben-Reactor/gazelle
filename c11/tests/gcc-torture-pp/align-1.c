# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/align-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/align-1.c" 2
void abort (void);

typedef int new_int __attribute__ ((aligned(16)));
struct S { int x; };

int main()
{
  if (sizeof(struct S) != sizeof(int))
    abort ();
  return 0;
}

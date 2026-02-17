# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20120817-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20120817-1.c" 2
void abort (void);
void exit (int);

typedef unsigned long long u64;
unsigned long foo = 0;
u64 f() __attribute__((noinline));

u64 f() {
  return ((u64)40) + ((u64) 24) * (int)(foo - 1);
}

int main ()
{
  if (f () != 16)
    abort ();
  exit (0);
}

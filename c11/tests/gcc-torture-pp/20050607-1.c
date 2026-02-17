# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20050607-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20050607-1.c" 2


extern void abort (void);

typedef int V2SI __attribute__ ((vector_size (8)));

int
main (void)
{


  if (((int)(long long)(V2SI){ 2, 2 }) != 2)
    abort ();

  return 0;
}

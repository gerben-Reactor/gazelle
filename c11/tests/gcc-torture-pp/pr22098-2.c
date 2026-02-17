# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr22098-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr22098-2.c" 2
extern void abort (void);
extern void exit (int);
typedef long unsigned int uintptr_t;
int
main (void)
{
  int a = 0;
  int *p;
  uintptr_t b;
  b = (uintptr_t)(p = &(int []){0, 1, 2}[1]);
  if (*p != 1 || *(int *)b != 1)
    abort ();
  exit (0);
}

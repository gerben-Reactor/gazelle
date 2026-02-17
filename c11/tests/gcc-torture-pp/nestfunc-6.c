# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/nestfunc-6.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/nestfunc-6.c" 2





typedef long unsigned int size_t;
extern void abort (void);
extern void exit (int);
extern void qsort(void *, size_t, size_t, int (*)(const void *, const void *));

int main ()
{
  __label__ nonlocal;
  int compare (const void *a, const void *b)
  {
    goto nonlocal;
  }

  char array[3];
  qsort (array, 3, 1, compare);
  abort ();

 nonlocal:
  exit (0);
}

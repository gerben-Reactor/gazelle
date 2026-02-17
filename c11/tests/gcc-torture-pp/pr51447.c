# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr51447.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr51447.c" 2




extern void abort (void);


register void *ptr asm ("rbx");




int
main (void)
{
  __label__ nonlocal_lab;

  void *save = ptr;

  __attribute__((noinline, noclone)) void
    bar (void *func)
      {
 ptr = func;
 goto nonlocal_lab;
      }
  bar (&&nonlocal_lab);
  return 1;
nonlocal_lab:
  if (ptr != &&nonlocal_lab)
    abort ();

  ptr = save;

  return 0;
}

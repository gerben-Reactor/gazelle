# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr64979.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr64979.c" 2


# 1 "/usr/lib/llvm-18/lib/clang/18/include/stdarg.h" 1 3
# 55 "/usr/lib/llvm-18/lib/clang/18/include/stdarg.h" 3
# 1 "/usr/lib/llvm-18/lib/clang/18/include/__stdarg___gnuc_va_list.h" 1 3
# 12 "/usr/lib/llvm-18/lib/clang/18/include/__stdarg___gnuc_va_list.h" 3
typedef __builtin_va_list __gnuc_va_list;
# 56 "/usr/lib/llvm-18/lib/clang/18/include/stdarg.h" 2 3




# 1 "/usr/lib/llvm-18/lib/clang/18/include/__stdarg_va_list.h" 1 3
# 12 "/usr/lib/llvm-18/lib/clang/18/include/__stdarg_va_list.h" 3
typedef __builtin_va_list va_list;
# 61 "/usr/lib/llvm-18/lib/clang/18/include/stdarg.h" 2 3




# 1 "/usr/lib/llvm-18/lib/clang/18/include/__stdarg_va_arg.h" 1 3
# 66 "/usr/lib/llvm-18/lib/clang/18/include/stdarg.h" 2 3




# 1 "/usr/lib/llvm-18/lib/clang/18/include/__stdarg___va_copy.h" 1 3
# 71 "/usr/lib/llvm-18/lib/clang/18/include/stdarg.h" 2 3




# 1 "/usr/lib/llvm-18/lib/clang/18/include/__stdarg_va_copy.h" 1 3
# 76 "/usr/lib/llvm-18/lib/clang/18/include/stdarg.h" 2 3
# 4 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr64979.c" 2

void __attribute__((noinline, noclone))
bar (int x, va_list *ap)
{
  if (ap)
    {
      int i;
      for (i = 0; i < 10; i++)
 if (i != __builtin_va_arg(*ap, int))
   __builtin_abort ();
      if (__builtin_va_arg(*ap, double) != 0.5)
 __builtin_abort ();
    }
}

void __attribute__((noinline, noclone))
foo (int x, ...)
{
  va_list ap;
  int n;

  __builtin_va_start(ap, x);
  n = __builtin_va_arg(ap, int);
  bar (x, (va_list *) ((n == 0) ? ((void *) 0) : &ap));
  __builtin_va_end(ap);
}

int
main ()
{
  foo (100, 1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0.5);
  return 0;
}

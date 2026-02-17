# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr44942.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr44942.c" 2


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
# 4 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr44942.c" 2

void
test1 (int a, int b, int c, int d, int e, int f, int g, long double h, ...)
{
  int i;
  va_list ap;

  __builtin_va_start(ap, h);
  i = __builtin_va_arg(ap, int);
  if (i != 1234)
    __builtin_abort ();
  __builtin_va_end(ap);
}

void
test2 (int a, int b, int c, int d, int e, int f, int g, long double h, int i,
       long double j, int k, long double l, int m, long double n, ...)
{
  int o;
  va_list ap;

  __builtin_va_start(ap, n);
  o = __builtin_va_arg(ap, int);
  if (o != 1234)
    __builtin_abort ();
  __builtin_va_end(ap);
}

void
test3 (double a, double b, double c, double d, double e, double f,
       double g, long double h, ...)
{
  double i;
  va_list ap;

  __builtin_va_start(ap, h);
  i = __builtin_va_arg(ap, double);
  if (i != 1234.0)
    __builtin_abort ();
  __builtin_va_end(ap);
}

void
test4 (double a, double b, double c, double d, double e, double f, double g,
       long double h, double i, long double j, double k, long double l,
       double m, long double n, ...)
{
  double o;
  va_list ap;

  __builtin_va_start(ap, n);
  o = __builtin_va_arg(ap, double);
  if (o != 1234.0)
    __builtin_abort ();
  __builtin_va_end(ap);
}

int
main ()
{
  test1 (0, 0, 0, 0, 0, 0, 0, 0.0L, 1234);
  test2 (0, 0, 0, 0, 0, 0, 0, 0.0L, 0, 0.0L, 0, 0.0L, 0, 0.0L, 1234);
  test3 (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0L, 1234.0);
  test4 (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0L, 0.0, 0.0L,
  0.0, 0.0L, 0.0, 0.0L, 1234.0);
  return 0;
}

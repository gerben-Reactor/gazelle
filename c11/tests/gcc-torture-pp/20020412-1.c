# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20020412-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20020412-1.c" 2




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
# 6 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20020412-1.c" 2

extern void abort (void);
extern void exit (int);

void bar (int c)
{
  static int d = '0';

  if (c != d++)
    abort ();
  if (c < '0' || c > '9')
    abort ();
}

void foo (int size, ...)
{
  struct
  {
    char x[size];
  } d;
  va_list ap;
  int i;

  __builtin_va_start(ap, size);
  d = __builtin_va_arg(ap, typeof (d));
  for (i = 0; i < size; i++)
    bar (d.x[i]);
  d = __builtin_va_arg(ap, typeof (d));
  for (i = 0; i < size; i++)
    bar (d.x[i]);
  __builtin_va_end(ap);
}

int main (void)
{
  int z = 5;
  struct { char a[z]; } x, y;

  x.a[0] = '0';
  x.a[1] = '1';
  x.a[2] = '2';
  x.a[3] = '3';
  x.a[4] = '4';
  y.a[0] = '5';
  y.a[1] = '6';
  y.a[2] = '7';
  y.a[3] = '8';
  y.a[4] = '9';
  foo (z, x, y);
  exit (0);
}

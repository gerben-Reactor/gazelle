# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr92904.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr92904.c" 2


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
# 4 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr92904.c" 2

struct S { long long a, b; };
struct __attribute__((aligned (16))) T { long long a, b; };
struct U { double a, b, c, d; };
struct __attribute__((aligned (32))) V { double a, b, c, d; };
struct W { double a; long long b; };
struct __attribute__((aligned (16))) X { double a; long long b; };

__int128 b;

struct S c;
struct T d;
struct U e;
struct V f;
struct W g;
struct X h;


__attribute__((noipa)) __int128
f1 (int x, ...)
{
  __int128 r;
  va_list ap;
  __builtin_va_start(ap, x);
  while (x--)
    __builtin_va_arg(ap, int);
  r = __builtin_va_arg(ap, __int128);
  __builtin_va_end(ap);
  return r;
}


__attribute__((noipa)) struct S
f2 (int x, ...)
{
  struct S r;
  va_list ap;
  __builtin_va_start(ap, x);
  while (x--)
    __builtin_va_arg(ap, int);
  r = __builtin_va_arg(ap, struct S);
  __builtin_va_end(ap);
  return r;
}

__attribute__((noipa)) struct T
f3 (int x, ...)
{
  struct T r;
  va_list ap;
  __builtin_va_start(ap, x);
  while (x--)
    __builtin_va_arg(ap, int);
  r = __builtin_va_arg(ap, struct T);
  __builtin_va_end(ap);
  return r;
}


__attribute__((noipa)) void
f4 (int x, ...)
{
  va_list ap;
  __builtin_va_start(ap, x);
  while (x--)
    __builtin_va_arg(ap, int);
  b = __builtin_va_arg(ap, __int128);
  __builtin_va_end(ap);
}


__attribute__((noipa)) void
f5 (int x, ...)
{
  va_list ap;
  __builtin_va_start(ap, x);
  while (x--)
    __builtin_va_arg(ap, int);
  c = __builtin_va_arg(ap, struct S);
  __builtin_va_end(ap);
}

__attribute__((noipa)) void
f6 (int x, ...)
{
  va_list ap;
  __builtin_va_start(ap, x);
  while (x--)
    __builtin_va_arg(ap, int);
  d = __builtin_va_arg(ap, struct T);
  __builtin_va_end(ap);
}

__attribute__((noipa)) struct U
f7 (int x, ...)
{
  struct U r;
  va_list ap;
  __builtin_va_start(ap, x);
  while (x--)
    __builtin_va_arg(ap, double);
  r = __builtin_va_arg(ap, struct U);
  __builtin_va_end(ap);
  return r;
}

__attribute__((noipa)) struct V
f8 (int x, ...)
{
  struct V r;
  va_list ap;
  __builtin_va_start(ap, x);
  while (x--)
    __builtin_va_arg(ap, double);
  r = __builtin_va_arg(ap, struct V);
  __builtin_va_end(ap);
  return r;
}

__attribute__((noipa)) void
f9 (int x, ...)
{
  va_list ap;
  __builtin_va_start(ap, x);
  while (x--)
    __builtin_va_arg(ap, double);
  e = __builtin_va_arg(ap, struct U);
  __builtin_va_end(ap);
}

__attribute__((noipa)) void
f10 (int x, ...)
{
  va_list ap;
  __builtin_va_start(ap, x);
  while (x--)
    __builtin_va_arg(ap, double);
  f = __builtin_va_arg(ap, struct V);
  __builtin_va_end(ap);
}

__attribute__((noipa)) struct W
f11 (int x, ...)
{
  struct W r;
  va_list ap;
  __builtin_va_start(ap, x);
  while (x--)
    {
      __builtin_va_arg(ap, int);
      __builtin_va_arg(ap, double);
    }
  r = __builtin_va_arg(ap, struct W);
  __builtin_va_end(ap);
  return r;
}

__attribute__((noipa)) struct X
f12 (int x, ...)
{
  struct X r;
  va_list ap;
  __builtin_va_start(ap, x);
  while (x--)
    {
      __builtin_va_arg(ap, int);
      __builtin_va_arg(ap, double);
    }
  r = __builtin_va_arg(ap, struct X);
  __builtin_va_end(ap);
  return r;
}

__attribute__((noipa)) void
f13 (int x, ...)
{
  va_list ap;
  __builtin_va_start(ap, x);
  while (x--)
    {
      __builtin_va_arg(ap, int);
      __builtin_va_arg(ap, double);
    }
  g = __builtin_va_arg(ap, struct W);
  __builtin_va_end(ap);
}

__attribute__((noipa)) void
f14 (int x, ...)
{
  va_list ap;
  __builtin_va_start(ap, x);
  while (x--)
    {
      __builtin_va_arg(ap, int);
      __builtin_va_arg(ap, double);
    }
  h = __builtin_va_arg(ap, struct X);
  __builtin_va_end(ap);
}

int
main ()
{
  union Y {

    __int128 b;

    struct S c;
    struct T d;
    struct U e;
    struct V f;
    struct W g;
    struct X h;
  } u, v;
  u.c.a = 0x5555555555555555ULL;
  u.c.b = 0xaaaaaaaaaaaaaaaaULL;







  v.b = f1 (0, u.b); do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);
  v.b = f1 (1, 0, u.b); do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);
  v.b = f1 (2, 0, 0, u.b); do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);
  v.b = f1 (3, 0, 0, 0, u.b); do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);
  v.b = f1 (4, 0, 0, 0, 0, u.b); do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);
  v.b = f1 (5, 0, 0, 0, 0, 0, u.b); do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);
  v.b = f1 (6, 0, 0, 0, 0, 0, 0, u.b); do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);
  v.b = f1 (7, 0, 0, 0, 0, 0, 0, 0, u.b); do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);
  v.b = f1 (8, 0, 0, 0, 0, 0, 0, 0, 0, u.b); do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);
  v.b = f1 (9, 0, 0, 0, 0, 0, 0, 0, 0, 0, u.b); do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);

  v.c = f2 (0, u.c); do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);
  v.c = f2 (1, 0, u.c); do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);
  v.c = f2 (2, 0, 0, u.c); do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);
  v.c = f2 (3, 0, 0, 0, u.c); do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);
  v.c = f2 (4, 0, 0, 0, 0, u.c); do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);
  v.c = f2 (5, 0, 0, 0, 0, 0, u.c); do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);
  v.c = f2 (6, 0, 0, 0, 0, 0, 0, u.c); do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);
  v.c = f2 (7, 0, 0, 0, 0, 0, 0, 0, u.c); do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);
  v.c = f2 (8, 0, 0, 0, 0, 0, 0, 0, 0, u.c); do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);
  v.c = f2 (9, 0, 0, 0, 0, 0, 0, 0, 0, 0, u.c); do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);
  v.d = f3 (0, u.d); do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);
  v.d = f3 (1, 0, u.d); do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);
  v.d = f3 (2, 0, 0, u.d); do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);
  v.d = f3 (3, 0, 0, 0, u.d); do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);
  v.d = f3 (4, 0, 0, 0, 0, u.d); do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);
  v.d = f3 (5, 0, 0, 0, 0, 0, u.d); do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);
  v.d = f3 (6, 0, 0, 0, 0, 0, 0, u.d); do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);
  v.d = f3 (7, 0, 0, 0, 0, 0, 0, 0, u.d); do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);
  v.d = f3 (8, 0, 0, 0, 0, 0, 0, 0, 0, u.d); do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);
  v.d = f3 (9, 0, 0, 0, 0, 0, 0, 0, 0, 0, u.d); do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);

  f4 (0, u.b); v.b = b; do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);
  f4 (1, 0, u.b); v.b = b; do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);
  f4 (2, 0, 0, u.b); v.b = b; do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);
  f4 (3, 0, 0, 0, u.b); v.b = b; do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);
  f4 (4, 0, 0, 0, 0, u.b); v.b = b; do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);
  f4 (5, 0, 0, 0, 0, 0, u.b); v.b = b; do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);
  f4 (6, 0, 0, 0, 0, 0, 0, u.b); v.b = b; do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);
  f4 (7, 0, 0, 0, 0, 0, 0, 0, u.b); v.b = b; do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);
  f4 (8, 0, 0, 0, 0, 0, 0, 0, 0, u.b); v.b = b; do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);
  f4 (9, 0, 0, 0, 0, 0, 0, 0, 0, 0, u.b); v.b = b; do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);

  f5 (0, u.c); v.c = c; do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);
  f5 (1, 0, u.c); v.c = c; do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);
  f5 (2, 0, 0, u.c); v.c = c; do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);
  f5 (3, 0, 0, 0, u.c); v.c = c; do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);
  f5 (4, 0, 0, 0, 0, u.c); v.c = c; do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);
  f5 (5, 0, 0, 0, 0, 0, u.c); v.c = c; do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);
  f5 (6, 0, 0, 0, 0, 0, 0, u.c); v.c = c; do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);
  f5 (7, 0, 0, 0, 0, 0, 0, 0, u.c); v.c = c; do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);
  f5 (8, 0, 0, 0, 0, 0, 0, 0, 0, u.c); v.c = c; do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);
  f5 (9, 0, 0, 0, 0, 0, 0, 0, 0, 0, u.c); v.c = c; do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);
  f6 (0, u.d); v.d = d; do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);
  f6 (1, 0, u.d); v.d = d; do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);
  f6 (2, 0, 0, u.d); v.d = d; do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);
  f6 (3, 0, 0, 0, u.d); v.d = d; do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);
  f6 (4, 0, 0, 0, 0, u.d); v.d = d; do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);
  f6 (5, 0, 0, 0, 0, 0, u.d); v.d = d; do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);
  f6 (6, 0, 0, 0, 0, 0, 0, u.d); v.d = d; do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);
  f6 (7, 0, 0, 0, 0, 0, 0, 0, u.d); v.d = d; do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);
  f6 (8, 0, 0, 0, 0, 0, 0, 0, 0, u.d); v.d = d; do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);
  f6 (9, 0, 0, 0, 0, 0, 0, 0, 0, 0, u.d); v.d = d; do { if (u.c.a != v.c.a || u.c.b != v.c.b) __builtin_abort (); u.c.a++; u.c.b--; } while (0);
  u.e.a = 1.25;
  u.e.b = 2.75;
  u.e.c = -3.5;
  u.e.d = -2.0;
# 305 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr92904.c"
  v.e = f7 (0, u.e); do { if (u.e.a != v.e.a || u.e.b != v.e.b || u.e.c != v.e.c || u.e.d != v.e.d) __builtin_abort (); u.e.a++; u.e.b--; u.e.c++; u.e.d--; } while (0);
  v.e = f7 (1, 0.0, u.e); do { if (u.e.a != v.e.a || u.e.b != v.e.b || u.e.c != v.e.c || u.e.d != v.e.d) __builtin_abort (); u.e.a++; u.e.b--; u.e.c++; u.e.d--; } while (0);
  v.e = f7 (2, 0.0, 0.0, u.e); do { if (u.e.a != v.e.a || u.e.b != v.e.b || u.e.c != v.e.c || u.e.d != v.e.d) __builtin_abort (); u.e.a++; u.e.b--; u.e.c++; u.e.d--; } while (0);
  v.e = f7 (3, 0.0, 0.0, 0.0, u.e); do { if (u.e.a != v.e.a || u.e.b != v.e.b || u.e.c != v.e.c || u.e.d != v.e.d) __builtin_abort (); u.e.a++; u.e.b--; u.e.c++; u.e.d--; } while (0);
  v.e = f7 (4, 0.0, 0.0, 0.0, 0.0, u.e); do { if (u.e.a != v.e.a || u.e.b != v.e.b || u.e.c != v.e.c || u.e.d != v.e.d) __builtin_abort (); u.e.a++; u.e.b--; u.e.c++; u.e.d--; } while (0);
  v.e = f7 (5, 0.0, 0.0, 0.0, 0.0, 0.0, u.e); do { if (u.e.a != v.e.a || u.e.b != v.e.b || u.e.c != v.e.c || u.e.d != v.e.d) __builtin_abort (); u.e.a++; u.e.b--; u.e.c++; u.e.d--; } while (0);
  v.e = f7 (6, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, u.e); do { if (u.e.a != v.e.a || u.e.b != v.e.b || u.e.c != v.e.c || u.e.d != v.e.d) __builtin_abort (); u.e.a++; u.e.b--; u.e.c++; u.e.d--; } while (0);
  v.e = f7 (7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, u.e); do { if (u.e.a != v.e.a || u.e.b != v.e.b || u.e.c != v.e.c || u.e.d != v.e.d) __builtin_abort (); u.e.a++; u.e.b--; u.e.c++; u.e.d--; } while (0);
  v.e = f7 (8, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, u.e); do { if (u.e.a != v.e.a || u.e.b != v.e.b || u.e.c != v.e.c || u.e.d != v.e.d) __builtin_abort (); u.e.a++; u.e.b--; u.e.c++; u.e.d--; } while (0);
  v.e = f7 (9, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, u.e); do { if (u.e.a != v.e.a || u.e.b != v.e.b || u.e.c != v.e.c || u.e.d != v.e.d) __builtin_abort (); u.e.a++; u.e.b--; u.e.c++; u.e.d--; } while (0);
  v.f = f8 (0, u.f); do { if (u.e.a != v.e.a || u.e.b != v.e.b || u.e.c != v.e.c || u.e.d != v.e.d) __builtin_abort (); u.e.a++; u.e.b--; u.e.c++; u.e.d--; } while (0);
  v.f = f8 (1, 0.0, u.f); do { if (u.e.a != v.e.a || u.e.b != v.e.b || u.e.c != v.e.c || u.e.d != v.e.d) __builtin_abort (); u.e.a++; u.e.b--; u.e.c++; u.e.d--; } while (0);
  v.f = f8 (2, 0.0, 0.0, u.f); do { if (u.e.a != v.e.a || u.e.b != v.e.b || u.e.c != v.e.c || u.e.d != v.e.d) __builtin_abort (); u.e.a++; u.e.b--; u.e.c++; u.e.d--; } while (0);
  v.f = f8 (3, 0.0, 0.0, 0.0, u.f); do { if (u.e.a != v.e.a || u.e.b != v.e.b || u.e.c != v.e.c || u.e.d != v.e.d) __builtin_abort (); u.e.a++; u.e.b--; u.e.c++; u.e.d--; } while (0);
  v.f = f8 (4, 0.0, 0.0, 0.0, 0.0, u.f); do { if (u.e.a != v.e.a || u.e.b != v.e.b || u.e.c != v.e.c || u.e.d != v.e.d) __builtin_abort (); u.e.a++; u.e.b--; u.e.c++; u.e.d--; } while (0);
  v.f = f8 (5, 0.0, 0.0, 0.0, 0.0, 0.0, u.f); do { if (u.e.a != v.e.a || u.e.b != v.e.b || u.e.c != v.e.c || u.e.d != v.e.d) __builtin_abort (); u.e.a++; u.e.b--; u.e.c++; u.e.d--; } while (0);
  v.f = f8 (6, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, u.f); do { if (u.e.a != v.e.a || u.e.b != v.e.b || u.e.c != v.e.c || u.e.d != v.e.d) __builtin_abort (); u.e.a++; u.e.b--; u.e.c++; u.e.d--; } while (0);
  v.f = f8 (7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, u.f); do { if (u.e.a != v.e.a || u.e.b != v.e.b || u.e.c != v.e.c || u.e.d != v.e.d) __builtin_abort (); u.e.a++; u.e.b--; u.e.c++; u.e.d--; } while (0);
  v.f = f8 (8, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, u.f); do { if (u.e.a != v.e.a || u.e.b != v.e.b || u.e.c != v.e.c || u.e.d != v.e.d) __builtin_abort (); u.e.a++; u.e.b--; u.e.c++; u.e.d--; } while (0);
  v.f = f8 (9, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, u.f); do { if (u.e.a != v.e.a || u.e.b != v.e.b || u.e.c != v.e.c || u.e.d != v.e.d) __builtin_abort (); u.e.a++; u.e.b--; u.e.c++; u.e.d--; } while (0);
  f9 (0, u.e); v.e = e; do { if (u.e.a != v.e.a || u.e.b != v.e.b || u.e.c != v.e.c || u.e.d != v.e.d) __builtin_abort (); u.e.a++; u.e.b--; u.e.c++; u.e.d--; } while (0);
  f9 (1, 0.0, u.e); v.e = e; do { if (u.e.a != v.e.a || u.e.b != v.e.b || u.e.c != v.e.c || u.e.d != v.e.d) __builtin_abort (); u.e.a++; u.e.b--; u.e.c++; u.e.d--; } while (0);
  f9 (2, 0.0, 0.0, u.e); v.e = e; do { if (u.e.a != v.e.a || u.e.b != v.e.b || u.e.c != v.e.c || u.e.d != v.e.d) __builtin_abort (); u.e.a++; u.e.b--; u.e.c++; u.e.d--; } while (0);
  f9 (3, 0.0, 0.0, 0.0, u.e); v.e = e; do { if (u.e.a != v.e.a || u.e.b != v.e.b || u.e.c != v.e.c || u.e.d != v.e.d) __builtin_abort (); u.e.a++; u.e.b--; u.e.c++; u.e.d--; } while (0);
  f9 (4, 0.0, 0.0, 0.0, 0.0, u.e); v.e = e; do { if (u.e.a != v.e.a || u.e.b != v.e.b || u.e.c != v.e.c || u.e.d != v.e.d) __builtin_abort (); u.e.a++; u.e.b--; u.e.c++; u.e.d--; } while (0);
  f9 (5, 0.0, 0.0, 0.0, 0.0, 0.0, u.e); v.e = e; do { if (u.e.a != v.e.a || u.e.b != v.e.b || u.e.c != v.e.c || u.e.d != v.e.d) __builtin_abort (); u.e.a++; u.e.b--; u.e.c++; u.e.d--; } while (0);
  f9 (6, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, u.e); v.e = e; do { if (u.e.a != v.e.a || u.e.b != v.e.b || u.e.c != v.e.c || u.e.d != v.e.d) __builtin_abort (); u.e.a++; u.e.b--; u.e.c++; u.e.d--; } while (0);
  f9 (7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, u.e); v.e = e; do { if (u.e.a != v.e.a || u.e.b != v.e.b || u.e.c != v.e.c || u.e.d != v.e.d) __builtin_abort (); u.e.a++; u.e.b--; u.e.c++; u.e.d--; } while (0);
  f9 (8, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, u.e); v.e = e; do { if (u.e.a != v.e.a || u.e.b != v.e.b || u.e.c != v.e.c || u.e.d != v.e.d) __builtin_abort (); u.e.a++; u.e.b--; u.e.c++; u.e.d--; } while (0);
  f9 (9, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, u.e); v.e = e; do { if (u.e.a != v.e.a || u.e.b != v.e.b || u.e.c != v.e.c || u.e.d != v.e.d) __builtin_abort (); u.e.a++; u.e.b--; u.e.c++; u.e.d--; } while (0);
  f10 (0, u.f); v.f = f; do { if (u.e.a != v.e.a || u.e.b != v.e.b || u.e.c != v.e.c || u.e.d != v.e.d) __builtin_abort (); u.e.a++; u.e.b--; u.e.c++; u.e.d--; } while (0);
  f10 (1, 0.0, u.f); v.f = f; do { if (u.e.a != v.e.a || u.e.b != v.e.b || u.e.c != v.e.c || u.e.d != v.e.d) __builtin_abort (); u.e.a++; u.e.b--; u.e.c++; u.e.d--; } while (0);
  f10 (2, 0.0, 0.0, u.f); v.f = f; do { if (u.e.a != v.e.a || u.e.b != v.e.b || u.e.c != v.e.c || u.e.d != v.e.d) __builtin_abort (); u.e.a++; u.e.b--; u.e.c++; u.e.d--; } while (0);
  f10 (3, 0.0, 0.0, 0.0, u.f); v.f = f; do { if (u.e.a != v.e.a || u.e.b != v.e.b || u.e.c != v.e.c || u.e.d != v.e.d) __builtin_abort (); u.e.a++; u.e.b--; u.e.c++; u.e.d--; } while (0);
  f10 (4, 0.0, 0.0, 0.0, 0.0, u.f); v.f = f; do { if (u.e.a != v.e.a || u.e.b != v.e.b || u.e.c != v.e.c || u.e.d != v.e.d) __builtin_abort (); u.e.a++; u.e.b--; u.e.c++; u.e.d--; } while (0);
  f10 (5, 0.0, 0.0, 0.0, 0.0, 0.0, u.f); v.f = f; do { if (u.e.a != v.e.a || u.e.b != v.e.b || u.e.c != v.e.c || u.e.d != v.e.d) __builtin_abort (); u.e.a++; u.e.b--; u.e.c++; u.e.d--; } while (0);
  f10 (6, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, u.f); v.f = f; do { if (u.e.a != v.e.a || u.e.b != v.e.b || u.e.c != v.e.c || u.e.d != v.e.d) __builtin_abort (); u.e.a++; u.e.b--; u.e.c++; u.e.d--; } while (0);
  f10 (7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, u.f); v.f = f; do { if (u.e.a != v.e.a || u.e.b != v.e.b || u.e.c != v.e.c || u.e.d != v.e.d) __builtin_abort (); u.e.a++; u.e.b--; u.e.c++; u.e.d--; } while (0);
  f10 (8, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, u.f); v.f = f; do { if (u.e.a != v.e.a || u.e.b != v.e.b || u.e.c != v.e.c || u.e.d != v.e.d) __builtin_abort (); u.e.a++; u.e.b--; u.e.c++; u.e.d--; } while (0);
  f10 (9, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, u.f); v.f = f; do { if (u.e.a != v.e.a || u.e.b != v.e.b || u.e.c != v.e.c || u.e.d != v.e.d) __builtin_abort (); u.e.a++; u.e.b--; u.e.c++; u.e.d--; } while (0);
  u.g.a = 9.5;
  u.g.b = 0x5555555555555555ULL;







  v.g = f11 (0, u.g); do { if (u.e.a != v.e.a || u.e.b != v.e.b) __builtin_abort (); u.e.a++; u.e.b--; } while (0);
  v.g = f11 (1, 0, 0.0, u.g); do { if (u.e.a != v.e.a || u.e.b != v.e.b) __builtin_abort (); u.e.a++; u.e.b--; } while (0);
  v.g = f11 (2, 0, 0.0, 0, 0.0, u.g); do { if (u.e.a != v.e.a || u.e.b != v.e.b) __builtin_abort (); u.e.a++; u.e.b--; } while (0);
  v.g = f11 (3, 0, 0.0, 0, 0.0, 0, 0.0, u.g); do { if (u.e.a != v.e.a || u.e.b != v.e.b) __builtin_abort (); u.e.a++; u.e.b--; } while (0);
  v.g = f11 (4, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, u.g); do { if (u.e.a != v.e.a || u.e.b != v.e.b) __builtin_abort (); u.e.a++; u.e.b--; } while (0);
  v.g = f11 (5, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, u.g); do { if (u.e.a != v.e.a || u.e.b != v.e.b) __builtin_abort (); u.e.a++; u.e.b--; } while (0);
  v.g = f11 (6, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, u.g); do { if (u.e.a != v.e.a || u.e.b != v.e.b) __builtin_abort (); u.e.a++; u.e.b--; } while (0);
  v.g = f11 (7, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, u.g); do { if (u.e.a != v.e.a || u.e.b != v.e.b) __builtin_abort (); u.e.a++; u.e.b--; } while (0);
  v.g = f11 (8, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, u.g); do { if (u.e.a != v.e.a || u.e.b != v.e.b) __builtin_abort (); u.e.a++; u.e.b--; } while (0);
  v.g = f11 (9, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, u.g); do { if (u.e.a != v.e.a || u.e.b != v.e.b) __builtin_abort (); u.e.a++; u.e.b--; } while (0);
  v.h = f12 (0, u.h); do { if (u.e.a != v.e.a || u.e.b != v.e.b) __builtin_abort (); u.e.a++; u.e.b--; } while (0);
  v.h = f12 (1, 0, 0.0, u.h); do { if (u.e.a != v.e.a || u.e.b != v.e.b) __builtin_abort (); u.e.a++; u.e.b--; } while (0);
  v.h = f12 (2, 0, 0.0, 0, 0.0, u.h); do { if (u.e.a != v.e.a || u.e.b != v.e.b) __builtin_abort (); u.e.a++; u.e.b--; } while (0);
  v.h = f12 (3, 0, 0.0, 0, 0.0, 0, 0.0, u.h); do { if (u.e.a != v.e.a || u.e.b != v.e.b) __builtin_abort (); u.e.a++; u.e.b--; } while (0);
  v.h = f12 (4, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, u.h); do { if (u.e.a != v.e.a || u.e.b != v.e.b) __builtin_abort (); u.e.a++; u.e.b--; } while (0);
  v.h = f12 (5, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, u.h); do { if (u.e.a != v.e.a || u.e.b != v.e.b) __builtin_abort (); u.e.a++; u.e.b--; } while (0);
  v.h = f12 (6, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, u.h); do { if (u.e.a != v.e.a || u.e.b != v.e.b) __builtin_abort (); u.e.a++; u.e.b--; } while (0);
  v.h = f12 (7, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, u.h); do { if (u.e.a != v.e.a || u.e.b != v.e.b) __builtin_abort (); u.e.a++; u.e.b--; } while (0);
  v.h = f12 (8, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, u.h); do { if (u.e.a != v.e.a || u.e.b != v.e.b) __builtin_abort (); u.e.a++; u.e.b--; } while (0);
  v.h = f12 (9, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, u.h); do { if (u.e.a != v.e.a || u.e.b != v.e.b) __builtin_abort (); u.e.a++; u.e.b--; } while (0);
  f13 (0, u.g); v.g = g; do { if (u.e.a != v.e.a || u.e.b != v.e.b) __builtin_abort (); u.e.a++; u.e.b--; } while (0);
  f13 (1, 0, 0.0, u.g); v.g = g; do { if (u.e.a != v.e.a || u.e.b != v.e.b) __builtin_abort (); u.e.a++; u.e.b--; } while (0);
  f13 (2, 0, 0.0, 0, 0.0, u.g); v.g = g; do { if (u.e.a != v.e.a || u.e.b != v.e.b) __builtin_abort (); u.e.a++; u.e.b--; } while (0);
  f13 (3, 0, 0.0, 0, 0.0, 0, 0.0, u.g); v.g = g; do { if (u.e.a != v.e.a || u.e.b != v.e.b) __builtin_abort (); u.e.a++; u.e.b--; } while (0);
  f13 (4, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, u.g); v.g = g; do { if (u.e.a != v.e.a || u.e.b != v.e.b) __builtin_abort (); u.e.a++; u.e.b--; } while (0);
  f13 (5, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, u.g); v.g = g; do { if (u.e.a != v.e.a || u.e.b != v.e.b) __builtin_abort (); u.e.a++; u.e.b--; } while (0);
  f13 (6, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, u.g); v.g = g; do { if (u.e.a != v.e.a || u.e.b != v.e.b) __builtin_abort (); u.e.a++; u.e.b--; } while (0);
  f13 (7, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, u.g); v.g = g; do { if (u.e.a != v.e.a || u.e.b != v.e.b) __builtin_abort (); u.e.a++; u.e.b--; } while (0);
  f13 (8, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, u.g); v.g = g; do { if (u.e.a != v.e.a || u.e.b != v.e.b) __builtin_abort (); u.e.a++; u.e.b--; } while (0);
  f13 (9, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, u.g); v.g = g; do { if (u.e.a != v.e.a || u.e.b != v.e.b) __builtin_abort (); u.e.a++; u.e.b--; } while (0);
  f14 (0, u.h); v.h = h; do { if (u.e.a != v.e.a || u.e.b != v.e.b) __builtin_abort (); u.e.a++; u.e.b--; } while (0);
  f14 (1, 0, 0.0, u.h); v.h = h; do { if (u.e.a != v.e.a || u.e.b != v.e.b) __builtin_abort (); u.e.a++; u.e.b--; } while (0);
  f14 (2, 0, 0.0, 0, 0.0, u.h); v.h = h; do { if (u.e.a != v.e.a || u.e.b != v.e.b) __builtin_abort (); u.e.a++; u.e.b--; } while (0);
  f14 (3, 0, 0.0, 0, 0.0, 0, 0.0, u.h); v.h = h; do { if (u.e.a != v.e.a || u.e.b != v.e.b) __builtin_abort (); u.e.a++; u.e.b--; } while (0);
  f14 (4, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, u.h); v.h = h; do { if (u.e.a != v.e.a || u.e.b != v.e.b) __builtin_abort (); u.e.a++; u.e.b--; } while (0);
  f14 (5, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, u.h); v.h = h; do { if (u.e.a != v.e.a || u.e.b != v.e.b) __builtin_abort (); u.e.a++; u.e.b--; } while (0);
  f14 (6, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, u.h); v.h = h; do { if (u.e.a != v.e.a || u.e.b != v.e.b) __builtin_abort (); u.e.a++; u.e.b--; } while (0);
  f14 (7, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, u.h); v.h = h; do { if (u.e.a != v.e.a || u.e.b != v.e.b) __builtin_abort (); u.e.a++; u.e.b--; } while (0);
  f14 (8, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, u.h); v.h = h; do { if (u.e.a != v.e.a || u.e.b != v.e.b) __builtin_abort (); u.e.a++; u.e.b--; } while (0);
  f14 (9, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, u.h); v.h = h; do { if (u.e.a != v.e.a || u.e.b != v.e.b) __builtin_abort (); u.e.a++; u.e.b--; } while (0);
  return 0;
}

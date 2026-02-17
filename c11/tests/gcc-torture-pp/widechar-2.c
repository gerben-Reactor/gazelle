# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/widechar-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/widechar-2.c" 2
# 1 "/usr/lib/llvm-18/lib/clang/18/include/stddef.h" 1 3
# 72 "/usr/lib/llvm-18/lib/clang/18/include/stddef.h" 3
# 1 "/usr/lib/llvm-18/lib/clang/18/include/__stddef_ptrdiff_t.h" 1 3
# 18 "/usr/lib/llvm-18/lib/clang/18/include/__stddef_ptrdiff_t.h" 3
typedef long int ptrdiff_t;
# 73 "/usr/lib/llvm-18/lib/clang/18/include/stddef.h" 2 3




# 1 "/usr/lib/llvm-18/lib/clang/18/include/__stddef_size_t.h" 1 3
# 18 "/usr/lib/llvm-18/lib/clang/18/include/__stddef_size_t.h" 3
typedef long unsigned int size_t;
# 78 "/usr/lib/llvm-18/lib/clang/18/include/stddef.h" 2 3
# 87 "/usr/lib/llvm-18/lib/clang/18/include/stddef.h" 3
# 1 "/usr/lib/llvm-18/lib/clang/18/include/__stddef_wchar_t.h" 1 3
# 24 "/usr/lib/llvm-18/lib/clang/18/include/__stddef_wchar_t.h" 3
typedef int wchar_t;
# 88 "/usr/lib/llvm-18/lib/clang/18/include/stddef.h" 2 3




# 1 "/usr/lib/llvm-18/lib/clang/18/include/__stddef_null.h" 1 3
# 93 "/usr/lib/llvm-18/lib/clang/18/include/stddef.h" 2 3
# 107 "/usr/lib/llvm-18/lib/clang/18/include/stddef.h" 3
# 1 "/usr/lib/llvm-18/lib/clang/18/include/__stddef_max_align_t.h" 1 3
# 19 "/usr/lib/llvm-18/lib/clang/18/include/__stddef_max_align_t.h" 3
typedef struct {
  long long __clang_max_align_nonce1
      __attribute__((__aligned__(__alignof__(long long))));
  long double __clang_max_align_nonce2
      __attribute__((__aligned__(__alignof__(long double))));
} max_align_t;
# 108 "/usr/lib/llvm-18/lib/clang/18/include/stddef.h" 2 3




# 1 "/usr/lib/llvm-18/lib/clang/18/include/__stddef_offsetof.h" 1 3
# 113 "/usr/lib/llvm-18/lib/clang/18/include/stddef.h" 2 3
# 2 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/widechar-2.c" 2

void abort(void);
void exit(int);

const wchar_t ws[] = L"foo";

int
main (void)
{
  if (ws[0] != L'f' || ws[1] != L'o' || ws[2] != L'o' || ws[3] != L'\0')
    abort();
  exit(0);
}

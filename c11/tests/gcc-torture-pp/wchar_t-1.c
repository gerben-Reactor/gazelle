# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/wchar_t-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/wchar_t-1.c" 2

typedef int wchar_t;
wchar_t x[] = L"Ä";
wchar_t y = L'Ä';
extern void abort (void);
extern void exit (int);

int main (void)
{
  if (sizeof (x) / sizeof (wchar_t) != 2)
    abort ();
  if (x[0] != L'Ä' || x[1] != L'\0')
    abort ();
  if (y != L'Ä')
    abort ();
  exit (0);
}

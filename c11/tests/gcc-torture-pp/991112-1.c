# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/991112-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/991112-1.c" 2



void abort(void);
int isprint (int);

int rl_show_char (int c) { return 0; }

int rl_character_len (int c, int pos)
{
  return isprint (c) ? 1 : 2;
}

int main(void)
{
  int (*x)(int, int) = rl_character_len;
  if (x('a', 1) != 1)
    abort();
  if (x('\002', 1) != 2)
    abort();
  return 0;
}

# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr37125.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr37125.c" 2
extern void abort (void);

static inline unsigned int
mod_rhs(int rhs)
{
  if (rhs == 0) return 1;
  return rhs;
}

void func_44 (unsigned int p_45);
void func_44 (unsigned int p_45)
{
  if (!((p_45 * -9) % mod_rhs (-9))) {
      abort();
  }
}

int main (void)
{
  func_44 (2);
  return 0;
}

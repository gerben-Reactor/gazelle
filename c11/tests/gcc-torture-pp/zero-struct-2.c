# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/zero-struct-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/zero-struct-2.c" 2
void abort (void);
int ii;
typedef struct {} raw_spinlock_t;
typedef struct {
  raw_spinlock_t raw_lock;
} spinlock_t;
raw_spinlock_t one_raw_spinlock (void)
{
  raw_spinlock_t raw_lock;
  ii++;
  return raw_lock;
}
int main(void)
{
  spinlock_t lock = (spinlock_t) { .raw_lock = one_raw_spinlock() };
  if (ii != 1)
    abort ();
  return 0;
}

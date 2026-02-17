# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/bitfld-9.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/bitfld-9.c" 2
struct mouse_button_str {
        unsigned char left : 1;
        unsigned char right : 1;
        unsigned char middle : 1;
        } button;

static char fct (struct mouse_button_str newbutton) __attribute__ ((__noipa__));
static char
fct (struct mouse_button_str newbutton)
{
  char l = newbutton.left;
  char r = newbutton.right;
  char m = newbutton.middle;
  return l && r && m;
}

int main(void)
{
  struct mouse_button_str newbutton1;
  newbutton1.left = 1;
  newbutton1.middle = 1;
  newbutton1.right = 1;
  if (!fct (newbutton1))
    __builtin_abort ();

  newbutton1.left = 0;
  newbutton1.middle = 1;
  newbutton1.right = 1;
  if (fct (newbutton1))
    __builtin_abort ();
  newbutton1.left = 1;
  newbutton1.middle = 0;
  newbutton1.right = 1;
  if (fct (newbutton1))
    __builtin_abort ();
  newbutton1.left = 1;
  newbutton1.middle = 1;
  newbutton1.right = 0;
  if (fct (newbutton1))
    __builtin_abort ();


  newbutton1.left = 1;
  newbutton1.middle = 0;
  newbutton1.right = 0;
  if (fct (newbutton1))
    __builtin_abort ();
  newbutton1.left = 0;
  newbutton1.middle = 1;
  newbutton1.right = 0;
  if (fct (newbutton1))
    __builtin_abort ();
  newbutton1.left = 0;
  newbutton1.middle = 0;
  newbutton1.right = 1;
  if (fct (newbutton1))
    __builtin_abort ();
  newbutton1.left = 0;
  newbutton1.middle = 0;
  newbutton1.right = 0;
  if (fct (newbutton1))
    __builtin_abort ();
}

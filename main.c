#include<stdio.h>
#include"./ncalc.h"


// gcc main.c -L ./target/release/ -lc_interop_lib -lm
// "-lm" tells linker to include the math library when linking your program.
int main() {
  char* c = "3 > 2\0";
  CResult result =  evaluate(c);
  if( result.bool_result != NULL)
  {
    printf("bool result found: %u", *result.bool_result);
    return 0;
  }
  if(result.int_result != NULL)
  {
    printf("int result is not null");
    return 0;
  }
  if(result.float_result != NULL)
  {
    printf("found float result: %f\n", *result.float_result);
    return 0;
  }
  if(result.error == NULL) {
    printf("found error null");
    return -1;
  }
  printf("Hello world\n");
  return 0;
}

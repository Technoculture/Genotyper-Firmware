#include <stdlib.h>
#include <unity.h>

#include "model.hpp"

using namespace Model;

void test_struct_gcode_constructs(void) {}

int main(int argc, char **argv) {
  UNITY_BEGIN();
  RUN_TEST(test_struct_gcode_constructs);
  UNITY_END();

  return 0;
}

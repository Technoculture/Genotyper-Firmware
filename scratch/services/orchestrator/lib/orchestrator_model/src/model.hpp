#ifndef MODEL_H____
#define MODEL_H____

#include "gcode.pb.h"
#include <stdlib.h>

namespace Model {

class COM {
public:
  static void receive_callback();
};

} // namespace Model

#endif // MODEL_H____

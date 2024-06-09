
module \top
  wire \prod_1
  wire \prod_2
  wire \prod_3
  wire input 1 \x1
  wire input 2 \x2
  wire width 4 input 3 \x3_6
  wire output 4 \y1
  wire output 5 \y2
  wire output 6 \y3
  cell $and \cell_and
    parameter \A_SIGNED 0
    parameter \A_WIDTH 1
    parameter \B_SIGNED 0
    parameter \B_WIDTH 1
    parameter \Y_WIDTH 1
    connect \A \x1
    connect \B \x2
    connect \Y \prod_1
  end
  cell $not \cell_not
    parameter \A_SIGNED 0
    parameter \A_WIDTH 1
    parameter \Y_WIDTH 1
    connect \A \x1
    connect \Y \prod_2
  end
  cell $reduce_and \rand
    parameter \A_SIGNED 0
    parameter \A_WIDTH 4
    parameter \Y_WIDTH 1
    connect \A \x3_6
    connect \Y \prod_3
  end
  connect \y1 \prod_1
  connect \y2 \prod_2
  connect \y3 \prod_3
end

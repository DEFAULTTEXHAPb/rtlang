module top (xvec, y);

    input wire [3:0] xvec;
    output wire y;

    assign y = &xvec;

endmodule
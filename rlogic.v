module ff_with_en_and_async_reset(clock, reset, enable, d, q);

input clock, reset, enable, d;

output reg q;

always @(posedge clock, posedge reset)

    if (reset)

        q <= 0;

    else if (enable)

        q <= d;

endmodule
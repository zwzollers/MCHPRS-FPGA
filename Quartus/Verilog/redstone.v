module redstone (tick, inputs, outputs);
input tick;
input [9:0] inputs;
output [9:0] outputs;
wire w100;
assign w100 = inputs[0];
wire w300;
assign w300 = inputs[1];
wire w500;
assign w500 = inputs[2];
wire w700;
assign w700 = inputs[3];
wire w900;
assign w900 = inputs[4];
wire w1100;
assign w1100 = inputs[5];
wire w1300;
assign w1300 = inputs[6];
wire w1500;
assign w1500 = inputs[7];
wire w103;
torch #(1'b1) c103 (.i_clk(tick), .i_in(w111|w122), .o_out(w103));
wire w303;
torch #(1'b1) c303 (.i_clk(tick), .i_in(w311|w322), .o_out(w303));
wire w503;
torch #(1'b1) c503 (.i_clk(tick), .i_in(w511|w522), .o_out(w503));
wire w703;
torch #(1'b1) c703 (.i_clk(tick), .i_in(w711|w722), .o_out(w703));
wire w903;
torch #(1'b1) c903 (.i_clk(tick), .i_in(w911|w922), .o_out(w903));
wire w1103;
torch #(1'b1) c1103 (.i_clk(tick), .i_in(w1111|w1122), .o_out(w1103));
wire w1303;
torch #(1'b1) c1303 (.i_clk(tick), .i_in(w1311|w1322), .o_out(w1303));
wire w1503;
torch #(1'b1) c1503 (.i_clk(tick), .i_in(w1511|w1522), .o_out(w1503));
wire w108;
torch #(1'b1) c108 (.i_clk(tick), .i_in(w116|w217), .o_out(w108));
wire w208;
assign outputs[0] = (w108|w308);
wire w308;
torch #(1'b1) c308 (.i_clk(tick), .i_in(w316|w217), .o_out(w308));
wire w508;
torch #(1'b1) c508 (.i_clk(tick), .i_in(w516|w617), .o_out(w508));
wire w608;
assign outputs[1] = (w508|w708);
wire w708;
torch #(1'b1) c708 (.i_clk(tick), .i_in(w716|w617), .o_out(w708));
wire w908;
torch #(1'b1) c908 (.i_clk(tick), .i_in(w916|w1017), .o_out(w908));
wire w1008;
assign outputs[2] = (w908|w1108);
wire w1108;
torch #(1'b1) c1108 (.i_clk(tick), .i_in(w1116|w1017), .o_out(w1108));
wire w1308;
torch #(1'b1) c1308 (.i_clk(tick), .i_in(w1316|w1417), .o_out(w1308));
wire w1408;
assign outputs[3] = (w1308|w1508);
wire w1508;
torch #(1'b1) c1508 (.i_clk(tick), .i_in(w1516|w1417), .o_out(w1508));
wire w111;
torch #(1'b0) c111 (.i_clk(tick), .i_in(w100), .o_out(w111));
wire w311;
torch #(1'b0) c311 (.i_clk(tick), .i_in(w300), .o_out(w311));
wire w511;
torch #(1'b0) c511 (.i_clk(tick), .i_in(w500), .o_out(w511));
wire w711;
torch #(1'b0) c711 (.i_clk(tick), .i_in(w700), .o_out(w711));
wire w911;
torch #(1'b0) c911 (.i_clk(tick), .i_in(w900), .o_out(w911));
wire w1111;
torch #(1'b0) c1111 (.i_clk(tick), .i_in(w1100), .o_out(w1111));
wire w1311;
torch #(1'b0) c1311 (.i_clk(tick), .i_in(w1300), .o_out(w1311));
wire w1511;
torch #(1'b0) c1511 (.i_clk(tick), .i_in(w1500), .o_out(w1511));
wire w013;
assign w013 = inputs[8];
wire w1614;
assign outputs[4] = (w1423|w1525);
wire w116;
torch #(1'b0) c116 (.i_clk(tick), .i_in(w013), .o_out(w116));
wire w316;
torch #(1'b0) c316 (.i_clk(tick), .i_in(w103|w303), .o_out(w316));
wire w516;
torch #(1'b0) c516 (.i_clk(tick), .i_in(w223|w325), .o_out(w516));
wire w716;
torch #(1'b0) c716 (.i_clk(tick), .i_in(w503|w703), .o_out(w716));
wire w916;
torch #(1'b0) c916 (.i_clk(tick), .i_in(w623|w725), .o_out(w916));
wire w1116;
torch #(1'b0) c1116 (.i_clk(tick), .i_in(w903|w1103), .o_out(w1116));
wire w1316;
torch #(1'b0) c1316 (.i_clk(tick), .i_in(w1023|w1125), .o_out(w1316));
wire w1516;
torch #(1'b0) c1516 (.i_clk(tick), .i_in(w1303|w1503), .o_out(w1516));
wire w217;
torch #(1'b1) c217 (.i_clk(tick), .i_in(w116|w316), .o_out(w217));
wire w617;
torch #(1'b1) c617 (.i_clk(tick), .i_in(w516|w716), .o_out(w617));
wire w1017;
torch #(1'b1) c1017 (.i_clk(tick), .i_in(w916|w1116), .o_out(w1017));
wire w1417;
torch #(1'b1) c1417 (.i_clk(tick), .i_in(w1316|w1516), .o_out(w1417));
wire w122;
torch #(1'b1) c122 (.i_clk(tick), .i_in(w111|w311), .o_out(w122));
wire w322;
torch #(1'b1) c322 (.i_clk(tick), .i_in(w111|w311), .o_out(w322));
wire w522;
torch #(1'b1) c522 (.i_clk(tick), .i_in(w511|w711), .o_out(w522));
wire w722;
torch #(1'b1) c722 (.i_clk(tick), .i_in(w511|w711), .o_out(w722));
wire w922;
torch #(1'b1) c922 (.i_clk(tick), .i_in(w911|w1111), .o_out(w922));
wire w1122;
torch #(1'b1) c1122 (.i_clk(tick), .i_in(w911|w1111), .o_out(w1122));
wire w1322;
torch #(1'b1) c1322 (.i_clk(tick), .i_in(w1311|w1511), .o_out(w1322));
wire w1522;
torch #(1'b1) c1522 (.i_clk(tick), .i_in(w1311|w1511), .o_out(w1522));
wire w223;
torch #(1'b1) c223 (.i_clk(tick), .i_in(w111|w311), .o_out(w223));
wire w623;
torch #(1'b1) c623 (.i_clk(tick), .i_in(w511|w711), .o_out(w623));
wire w1023;
torch #(1'b1) c1023 (.i_clk(tick), .i_in(w911|w1111), .o_out(w1023));
wire w1423;
torch #(1'b1) c1423 (.i_clk(tick), .i_in(w1311|w1511), .o_out(w1423));
wire w325;
torch #(1'b1) c325 (.i_clk(tick), .i_in(w116|w316), .o_out(w325));
wire w725;
torch #(1'b1) c725 (.i_clk(tick), .i_in(w516|w716), .o_out(w725));
wire w1125;
torch #(1'b1) c1125 (.i_clk(tick), .i_in(w916|w1116), .o_out(w1125));
wire w1525;
torch #(1'b1) c1525 (.i_clk(tick), .i_in(w1316|w1516), .o_out(w1525));
endmodule
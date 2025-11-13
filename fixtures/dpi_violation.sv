module dpi_violation;
  import "DPI" function void bad_import();
  export "DPI" function bad_export = helper;
endmodule

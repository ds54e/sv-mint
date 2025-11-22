function logic add(
  input logic a,
  input logic b,
  output logic c,
  inout logic d,
  ref logic e
);
  c = a + b + d + e;
  return c;
endfunction

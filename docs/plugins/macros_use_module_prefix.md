# macros_use_module_prefix

## Script
- `plugins/macros_use_module_prefix.raw.py`

## Description
- Module-local macros must be prefixed with the module name

## Good

```systemverilog
module my_module;
  `define MY_MODULE_MACRO
endmodule
```

## Bad

```systemverilog
module my_module;
  `define MACRO
endmodule
```

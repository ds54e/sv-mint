from lib.typedef_naming_cst import violations_for


def check(req):
    return violations_for(req, "typedef_enum_value_upper")

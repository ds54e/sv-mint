from lib.typedef_naming_cst import violations_for


def check(req):
    return violations_for(req, "typedef.enum_name_lower_snake_e")

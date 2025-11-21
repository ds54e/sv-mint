from lib.typedef_naming_cst import violations_for


def check(req):
    return violations_for(req, "typedef_type_name_lower_snake_t")

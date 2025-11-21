from lib.typedef_naming_cst import violations_for


def check(req):
    return violations_for(req, "enum_type_names_lower_snake_e")

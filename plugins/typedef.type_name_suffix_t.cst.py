from lib.typedef_naming_cst import violations_for


def check(req):
    return violations_for(req, "typedef.type_name_suffix_t")

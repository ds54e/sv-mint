from lib.typedef_naming_ruleset import violations_for


def check(req):
    return violations_for(req, "typedef.enum_value_case")

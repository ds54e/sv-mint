from lib.default_nettype_ruleset import violations_for


def check(req):
    return violations_for(req, "default_nettype_ends_with_wire")

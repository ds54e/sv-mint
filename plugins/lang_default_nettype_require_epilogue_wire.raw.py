from lib.default_nettype_ruleset import violations_for


def check(req):
    return violations_for(req, "lang_default_nettype_require_epilogue_wire")

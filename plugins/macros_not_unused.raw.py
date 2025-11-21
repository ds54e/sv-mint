from lib.macro_usage_ruleset import violations_for


def check(req):
    return violations_for(req, "macros_not_unused")

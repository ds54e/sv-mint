from lib.naming_ruleset import violations_for


def check(req):
    return violations_for(req, "naming.no_numeric_suffix")

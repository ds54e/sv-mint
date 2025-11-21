from lib.package_ruleset import violations_for


def check(req):
    return violations_for(req, "package.require_defines_in_pkg")

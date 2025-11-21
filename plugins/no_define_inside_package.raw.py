from lib.package_ruleset import violations_for


def check(req):
    return violations_for(req, "no_define_inside_package")

from lib.package_ruleset import violations_for


def check(req):
    return violations_for(req, "package.no_define_in_package")

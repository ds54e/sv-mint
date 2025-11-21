from lib.package_ruleset import violations_for


def check(req):
    return violations_for(req, "package.single_package")

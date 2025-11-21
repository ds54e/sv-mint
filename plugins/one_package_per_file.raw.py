from lib.package_ruleset import violations_for


def check(req):
    return violations_for(req, "one_package_per_file")

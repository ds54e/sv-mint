from lib.naming_ruleset import violations_for


def check(req):
    return violations_for(req, "parameter_names_uppercase")

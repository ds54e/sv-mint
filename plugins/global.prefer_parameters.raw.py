from lib.global_define_ruleset import violations_for


def check(req):
    return violations_for(req, "global.prefer_parameters")

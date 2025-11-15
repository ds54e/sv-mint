from lib.lang_construct_ruleset import violations_for


def check(req):
    return violations_for(req, "lang.no_delays")

from lib.lang_construct_ruleset import violations_for


def check(req):
    return violations_for(req, "lang.always_ff_reset")

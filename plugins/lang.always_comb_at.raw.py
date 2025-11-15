from lib.lang_construct_ruleset import violations_for


def check(req):
    return violations_for(req, "lang.always_comb_at")

from lib.dv_text_ruleset import violations_for


def check(req):
    return violations_for(req, "rand.dv_macro_with_required")

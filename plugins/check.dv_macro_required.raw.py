from lib.dv_text_ruleset import violations_for


def check(req):
    return violations_for(req, "check.dv_macro_required")

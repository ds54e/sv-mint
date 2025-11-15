from lib.dv_text_ruleset import violations_for


def check(req):
    return violations_for(req, "style.function_scope")

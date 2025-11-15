from lib.format_text_ruleset import violations_for


def check(req):
    return violations_for(req, "format.no_tabs")

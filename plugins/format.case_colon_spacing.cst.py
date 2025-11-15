from lib.format_spacing_ruleset import violations_for


def check(req):
    return violations_for(req, "format.case_colon_spacing")

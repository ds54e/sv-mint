from lib.format_indent_ruleset import violations_for


def check(req):
    return violations_for(req, "format.line_continuation_right")

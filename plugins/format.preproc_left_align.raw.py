from lib.format_indent_ruleset import violations_for


def check(req):
    return violations_for(req, "format.preproc_left_align")

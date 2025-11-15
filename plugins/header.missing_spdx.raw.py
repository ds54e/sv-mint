from lib.header_comment_ruleset import violations_for


def check(req):
    return violations_for(req, "header.missing_spdx")

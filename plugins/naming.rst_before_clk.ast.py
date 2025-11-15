from lib.naming_ruleset import violations_for


def check(req):
    return violations_for(req, "naming.rst_before_clk")

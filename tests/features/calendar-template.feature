Feature: bullet calendar
    As a bullet journaler, I want to be able to output a list of days for a month

    Scenario: Valid Month
    When I run the command "bullet calendar 2016-04"
    Then I should see the following output:
    """
    * `01 S:`
    * `02 M:`
    * `03 T:`
    * `04 W:`
    ...
    * `30 M:`
    """

    Scenario: Invalid Month String
    When I run the command "bullet calendar invalid"
    Then I should see the following error:
    """
    Invalid date string, "invalid". Use the format "YYYY-MM"
    """

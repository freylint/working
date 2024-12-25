Feature: Application configuration support

  Scenario: Load config from file
    Given Valid config file exists
    When I load the configs
    Then Config matches config file

  Scenario: Load config from CLI
    Given Clargs exist
    When I load the configs
    Then Config matches clargs

  Scenario: CLI overrules config file
    Given Valid config file exists
    Given Clargs exist
    When I load the configs
    Then CLArgs overrule config file

  Scenario: Defaults written when no config
    Given No config file
    When I load the configs
    Then Default config file created

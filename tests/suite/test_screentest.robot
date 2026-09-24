*** Settings ***
Documentation    Test cases for screen-test snap
Resource         kvm.resource


*** Test Cases ***
Screen Test Launches And Renders
    [Documentation]    Verify screen-test snap launches and renders a UI on Mir
    [Tags]    smoke    yarf:certification_status: blocker
    Log Screenshot

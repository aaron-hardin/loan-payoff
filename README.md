# loan-payoff

App to show which order you should pay loans off in

# Disclaimer

This application is intended as a fun project and makes a lot of assumptions about the type of loans, please do not use this as financial advice.

# CLI

The CLI takes two arguments: file path, and extra payment amount (optional, default: 100.00)

# TODO
- Error checking/feedback to user when entered information doesn't match up
   - Should also cover the case where the loans would never be paid off
   - Should not show errors until the user tries to do something with the invalid data. Added some initial validation but it shows errors as soon as a new loan is added
- Add unit tests (github ci should run them)
- Remove pre-filled loans (should all be user entered)
use crate::helper::*;

use flowy_user::{errors::*, event::UserEvent::*, prelude::*};
use serial_test::*;

#[test]
#[serial]
fn sign_up_success() {
    let _ = UserEventTester::new(SignOut).sync_send();
    let request = SignUpRequest {
        email: valid_email(),
        name: valid_name(),
        password: valid_password(),
    };

    let _response = UserEventTester::new(SignUp).request(request).sync_send();
    // .parse::<SignUpResponse>();
    // dbg!(&response);
}

#[test]
fn sign_up_with_invalid_email() {
    for email in invalid_email_test_case() {
        let request = SignUpRequest {
            email: email.to_string(),
            name: valid_name(),
            password: valid_password(),
        };

        assert_eq!(
            UserEventTester::new(SignUp)
                .request(request)
                .sync_send()
                .error()
                .code,
            UserErrorCode::EmailInvalid
        );
    }
}
#[test]
fn sign_up_with_invalid_password() {
    for password in invalid_password_test_case() {
        let request = SignUpRequest {
            email: valid_email(),
            name: valid_name(),
            password,
        };

        assert_eq!(
            UserEventTester::new(SignUp)
                .request(request)
                .sync_send()
                .error()
                .code,
            UserErrorCode::PasswordInvalid
        );
    }
}
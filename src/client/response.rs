use chrono::{DateTime, FixedOffset};

use chrono::Utc;
use std::time::Duration;
use std::{convert::{TryFrom, TryInto}, str::FromStr};

/// FCM errors which have HTTP status code defined.
///
/// Check <https://firebase.google.com/docs/reference/fcm/rest/v1/ErrorCode>
/// for more information.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u16)]
pub enum FcmHttpError {
    InvalidArgument = 400,
    Unregistered = 404,
    SenderIdMismatch = 403,
    QuotaExceeded = 429,
    Unavailable = 503,
    Internal = 500,
    ThirdPartyAuthError = 401,
}

impl TryFrom<u16> for FcmHttpError {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            400 => Ok(FcmHttpError::InvalidArgument),
            404 => Ok(FcmHttpError::Unregistered),
            403 => Ok(FcmHttpError::SenderIdMismatch),
            429 => Ok(FcmHttpError::QuotaExceeded),
            503 => Ok(FcmHttpError::Unavailable),
            500 => Ok(FcmHttpError::Internal),
            401 => Ok(FcmHttpError::ThirdPartyAuthError),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FcmHttpResponseStatus {
    /// FCM message was sent successfully.
    Ok,
    /// HTTP error code was detected as FCM error.
    Error(FcmHttpError),
    /// HTTP status code did not match with [FcmHttpError] and
    /// `FcmError` JSON was detected.
    ///
    /// <https://firebase.google.com/docs/reference/fcm/rest/v1/FcmError>
    ///
    /// This variant is named as [FcmHttpResponseStatus::UnspecifiedError]
    /// because `UNSPECIFIED_ERROR` is the only `ErrorCode` variant that has no
    /// corresponding HTTP status code.
    ///
    /// <https://firebase.google.com/docs/reference/fcm/rest/v1/ErrorCode>
    UnspecifiedError,
    Unknown {
        http_status_code: u16,
    },
}

impl FcmHttpResponseStatus {
    pub fn new(
        http_status_code: u16,
        response_json: &serde_json::Map<String, serde_json::Value>,
    ) -> Self {
        if let Ok(error) = http_status_code.try_into() {
            return FcmHttpResponseStatus::Error(error);
        }

        let message = response_json.get("name");
        let fcm_error = response_json.get("error_code");
        match (message, fcm_error) {
            (Some(_), _) => FcmHttpResponseStatus::Ok,
            (_, Some(_)) => FcmHttpResponseStatus::UnspecifiedError,
            (None, None) => FcmHttpResponseStatus::Unknown {
                http_status_code,
            },
        }
    }
}

/// HTTP `Retry-After` header value.
#[derive(Debug, Clone, PartialEq)]
pub enum RetryAfter {
    /// Amount of time to wait until retrying the message is allowed.
    Delay(Duration),

    /// A point in time until retrying the message is allowed.
    DateTime(DateTime<FixedOffset>),
}

impl RetryAfter {
    /// Wait time calculated from current operating system time.
    pub fn wait_time(&self) -> Duration {
        self.wait_time_with_current_time(Utc::now().fixed_offset())
    }

    fn wait_time_with_current_time(&self, now: DateTime<FixedOffset>) -> Duration {
        match *self {
            RetryAfter::Delay(duration) => duration,
            RetryAfter::DateTime(date_time) =>
                (date_time - now)
                    .to_std()
                    // TimeDelta is negative when the date_time is in the
                    // past. In that case wait time is 0.
                    .unwrap_or(Duration::ZERO)
        }
    }
}

impl FromStr for RetryAfter {
    type Err = chrono::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<u64>()
            .map(Duration::from_secs)
            .map(RetryAfter::Delay)
            .or_else(
                |_| DateTime::parse_from_rfc2822(s)
                    .map(RetryAfter::DateTime)
            )
    }
}

#[derive(Debug, Clone)]
pub struct FcmResponse {
    response_status: FcmHttpResponseStatus,
    response_json_object: serde_json::Map<String, serde_json::Value>,
    retry_after: Option<RetryAfter>,
}

impl FcmResponse {
    pub(crate) fn new(
        response_status: FcmHttpResponseStatus,
        response_json_object: serde_json::Map<String, serde_json::Value>,
        retry_after: Option<RetryAfter>,
    ) -> Self {
        Self {
            response_status,
            response_json_object,
            retry_after,
        }
    }

    pub fn recommended_action(&self) -> Option<RecomendedAction> {
        RecomendedAction::analyze(self)
    }

    pub fn status(&self) -> FcmHttpResponseStatus {
        self.response_status
    }

    pub fn json(&self) -> &serde_json::Map<String, serde_json::Value> {
        &self.response_json_object
    }

    pub fn retry_after(&self) -> Option<&RetryAfter> {
        self.retry_after.as_ref()
    }
}


/// Action which server or developer should do based on the [FcmResponse].
///
/// Check <https://firebase.google.com/docs/reference/fcm/rest/v1/ErrorCode>
/// and <https://firebase.google.com/docs/cloud-messaging/scale-fcm#handling-retries>
/// for more details.
#[derive(Debug, Clone, PartialEq)]
pub enum RecomendedAction<'a> {
    /// Error [FcmHttpError::Unregistered] was received.
    /// The app token sent with the message was detected as
    /// missing or unregistered and should be removed.
    RemoveFcmAppToken,

    /// Error [FcmHttpError::InvalidArgument] was received. Check
    /// that the sent message is correct.
    FixMessageContent,

    /// Error [FcmHttpError::SenderIdMismatch] was received. Check
    /// that that client and server uses the same sender ID.
    CheckSenderIdEquality,

    /// Error [FcmHttpError::QuotaExceeded] was received. Reduce
    /// overall message sending rate, device message rate or
    /// topic message rate and then retry sending the previous
    /// message.
    ///
    /// TODO: Figure out QuotaExceeded format to know what quota was exceeded
    ReduceMessageRateAndRetry(RecomendedWaitTime<'a>),

    /// Error [FcmHttpError::Unavailable] or [FcmHttpError::Internal]
    /// was received. Wait specific amount of time before retrying the message.
    Retry(RecomendedWaitTime<'a>),

    /// Error [FcmHttpError::ThirdPartyAuthError] was received. Check
    /// credentials related to iOS and web push notifications.
    CheckIosAndWebCredentials,
}

impl RecomendedAction<'_> {
    fn analyze(response: &FcmResponse) -> Option<RecomendedAction> {
        match response.status() {
            FcmHttpResponseStatus::Ok |
            FcmHttpResponseStatus::UnspecifiedError |
            FcmHttpResponseStatus::Unknown { .. } => None,
            FcmHttpResponseStatus::Error(e) => match e {
                FcmHttpError::Unregistered => Some(RecomendedAction::RemoveFcmAppToken),
                FcmHttpError::InvalidArgument => Some(RecomendedAction::FixMessageContent),
                FcmHttpError::SenderIdMismatch =>
                    Some(RecomendedAction::CheckSenderIdEquality),
                FcmHttpError::QuotaExceeded => {
                    let wait_time = if let Some(ra) = response.retry_after() {
                        RecomendedWaitTime::SpecificWaitTime(ra)
                    } else {
                        RecomendedWaitTime::InitialWaitTime(Duration::from_secs(60))
                    };

                    Some(RecomendedAction::ReduceMessageRateAndRetry(wait_time))
                }
                FcmHttpError::Unavailable => {
                    let wait_time = if let Some(ra) = response.retry_after() {
                        RecomendedWaitTime::SpecificWaitTime(ra)
                    } else {
                        RecomendedWaitTime::InitialWaitTime(Duration::from_secs(10))
                    };

                    Some(RecomendedAction::Retry(wait_time))
                }
                FcmHttpError::Internal =>
                    Some(RecomendedAction::Retry(
                        RecomendedWaitTime::InitialWaitTime(Duration::from_secs(10))
                    )),
                FcmHttpError::ThirdPartyAuthError =>
                    Some(RecomendedAction::CheckIosAndWebCredentials),
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RecomendedWaitTime<'a> {
    /// Initial wait time for exponential back-off.
    ///
    /// If the next request will be initial retry then wait this
    /// amount of time before sending the request. For next retries
    /// multiply the wait time by itself (then the wait time
    /// grows exponentially).
    ///
    /// Note also that Google documentation also recommends implementing
    /// jittering to exponential back-off.
    InitialWaitTime(Duration),

    /// Specific wait time from HTTP header.
    SpecificWaitTime(&'a RetryAfter),
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::DateTime;

    #[test]
    fn test_retry_after_from_seconds() {
        let expected_wait_time = Duration::from_secs(1);
        let expected = RetryAfter::Delay(expected_wait_time);
        assert_eq!(expected, "1".parse().unwrap());
        assert_eq!(expected_wait_time, expected.wait_time_with_current_time(DateTime::default()));
    }

    #[test]
    fn test_retry_after_from_date() {
        let date = "Sun, 06 Nov 1994 08:49:37 GMT";
        let date_time = DateTime::parse_from_rfc2822(date).unwrap();
        let retry_after = RetryAfter::from_str(date).unwrap();

        assert_eq!(
            RetryAfter::DateTime(date_time),
            retry_after,
        );

        assert_eq!(
            Duration::ZERO,
            retry_after.wait_time_with_current_time(date_time),
        );
    }

    #[test]
    fn test_retry_after_from_date_and_get_wait_time_using_future_date() {
        let date = "Sun, 06 Nov 1994 08:49:37 GMT";
        let retry_after = RetryAfter::from_str(date).unwrap();
        let future_date = "Sun, 06 Nov 1994 08:49:38 GMT";
        let future_date_time = DateTime::parse_from_rfc2822(future_date).unwrap();

        assert_eq!(
            Duration::from_secs(0),
            retry_after.wait_time_with_current_time(future_date_time),
        );
    }

    #[test]
    fn test_retry_after_from_date_and_get_wait_time_using_past_date() {
        let date = "Sun, 06 Nov 1994 08:49:37 GMT";
        let retry_after = RetryAfter::from_str(date).unwrap();
        let past_date = "Sun, 06 Nov 1994 08:49:36 GMT";
        let past_date_time = DateTime::parse_from_rfc2822(past_date).unwrap();

        assert_eq!(
            Duration::from_secs(1),
            retry_after.wait_time_with_current_time(past_date_time),
        );
    }

    #[test]
    fn test_retry_after_from_date_and_get_wait_time_using_different_timezone() {
        let date = "Sun, 06 Nov 1994 08:49:37 GMT";
        let retry_after = RetryAfter::from_str(date).unwrap();
        let past_date = "Sun, 06 Nov 1994 08:49:37 +0100";
        let past_date_time = DateTime::parse_from_rfc2822(past_date).unwrap();

        assert_eq!(
            Duration::from_secs(60 * 60),
            retry_after.wait_time_with_current_time(past_date_time),
        );
    }
}

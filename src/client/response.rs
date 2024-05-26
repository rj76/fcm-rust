use chrono::{DateTime, FixedOffset};

use chrono::Utc;
use std::time::Duration;
use std::{
    convert::{TryFrom, TryInto},
    str::FromStr,
};

/// Error cases which can be detected from [FcmResponse].
///
/// Check <https://firebase.google.com/docs/reference/fcm/rest/v1/ErrorCode>
/// for more information.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FcmResponseError {
    /// HTTP 400
    InvalidArgument,
    /// HTTP 404
    Unregistered,
    /// HTTP 403
    SenderIdMismatch,
    /// HTTP 429
    QuotaExceeded,
    /// HTTP 503
    Unavailable,
    /// HTTP 500
    Internal,
    /// HTTP 401
    ThirdPartyAuth,
    /// `UNSPECIFIED_ERROR` (no HTTP error code defined)
    Unspecified,
    /// Response is not successful and API reference does not have
    /// matching error.
    Unknown,
}

impl FcmResponseError {
    pub fn detect_from(
        http_status_code: u16,
        response_json: &serde_json::Map<String, serde_json::Value>,
    ) -> Option<Self> {
        if let Ok(error) = http_status_code.try_into() {
            Some(error)
        } else if Self::get_error(response_json) == Some("UNSPECIFIED_ERROR") {
            Some(Self::Unspecified)
        } else if response_json.get("name").is_none() {
            Some(Self::Unknown)
        } else {
            None // No error
        }
    }

    fn get_error(response_json: &serde_json::Map<String, serde_json::Value>) -> Option<&str> {
        Self::get_error_using_api_reference(response_json)
            .or_else(|| Self::get_error_using_real_response(response_json))
    }

    /// Currently (2024-05-26) FCM API response JSON does not have
    /// this location for INVALID_ARGUMENT error.
    fn get_error_using_api_reference(response_json: &serde_json::Map<String, serde_json::Value>) -> Option<&str> {
        response_json.get("error_code").and_then(|v| v.as_str())
    }

    /// Current (2024-05-26) FCM API response JSON location for
    /// INVALID_ARGUMENT error and possibly for the other errors
    /// as well.
    fn get_error_using_real_response(response_json: &serde_json::Map<String, serde_json::Value>) -> Option<&str> {
        response_json
            .get("error")
            .and_then(|v| v.get("status"))
            .and_then(|v| v.as_str())
    }
}

impl TryFrom<u16> for FcmResponseError {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            400 => Ok(Self::InvalidArgument),
            404 => Ok(Self::Unregistered),
            403 => Ok(Self::SenderIdMismatch),
            429 => Ok(Self::QuotaExceeded),
            503 => Ok(Self::Unavailable),
            500 => Ok(Self::Internal),
            401 => Ok(Self::ThirdPartyAuth),
            _ => Err(()),
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
        self.wait_time_with_time_provider(|| Utc::now().fixed_offset())
    }

    fn wait_time_with_time_provider(&self, get_time: impl FnOnce() -> DateTime<FixedOffset>) -> Duration {
        match *self {
            RetryAfter::Delay(duration) => duration,
            RetryAfter::DateTime(date_time) => (date_time - get_time())
                .to_std()
                // TimeDelta is negative when the date_time is in the
                // past. In that case wait time is 0.
                .unwrap_or(Duration::ZERO),
        }
    }
}

impl FromStr for RetryAfter {
    type Err = chrono::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<u64>()
            .map(Duration::from_secs)
            .map(RetryAfter::Delay)
            .or_else(|_| DateTime::parse_from_rfc2822(s).map(RetryAfter::DateTime))
    }
}

#[derive(Debug, Clone)]
pub struct FcmResponse {
    http_status_code: u16,
    response_json_object: serde_json::Map<String, serde_json::Value>,
    retry_after: Option<RetryAfter>,
}

impl FcmResponse {
    pub(crate) fn new(
        http_status_code: u16,
        response_json_object: serde_json::Map<String, serde_json::Value>,
        retry_after: Option<RetryAfter>,
    ) -> Self {
        Self {
            http_status_code,
            response_json_object,
            retry_after,
        }
    }

    /// If `None` then [crate::message::Message] is sent successfully.
    pub fn recommended_error_handling_action(&self) -> Option<RecomendedAction> {
        RecomendedAction::analyze(self)
    }

    /// If `None` then [crate::message::Message] is sent successfully.
    pub fn error(&self) -> Option<FcmResponseError> {
        FcmResponseError::detect_from(self.http_status_code, &self.response_json_object)
    }

    pub fn http_status_code(&self) -> u16 {
        self.http_status_code
    }

    pub fn json(&self) -> &serde_json::Map<String, serde_json::Value> {
        &self.response_json_object
    }

    pub fn retry_after(&self) -> Option<&RetryAfter> {
        self.retry_after.as_ref()
    }
}

/// Error handling action which server or developer should do based on
/// [FcmResponseError] and possible [RetryAfter].
///
/// Check <https://firebase.google.com/docs/reference/fcm/rest/v1/ErrorCode>
/// and <https://firebase.google.com/docs/cloud-messaging/scale-fcm#handling-retries>
/// for more details.
#[derive(Debug, Clone, PartialEq)]
pub enum RecomendedAction<'a> {
    /// Error [FcmResponseError::Unregistered] was detected.
    /// The app token sent with the message was detected as
    /// missing or unregistered and should be removed.
    RemoveFcmAppToken,

    /// Error [FcmResponseError::InvalidArgument] was detected. Check
    /// that the sent message is correct.
    FixMessageContent,

    /// Error [FcmResponseError::SenderIdMismatch] was detected. Check
    /// that that client and server uses the same sender ID.
    CheckSenderIdEquality,

    /// Error [FcmResponseError::QuotaExceeded] was detected. Reduce
    /// overall message sending rate, device message rate or
    /// topic message rate. After that check [RecomendedWaitTime] to determine
    /// should specific or exponential back-off wait time should be used as
    /// a waiting time. After the waiting time is elapsed then resend the
    /// previous message.
    ///
    /// TODO: Figure out QuotaExceeded format to know what quota was exceeded
    ReduceMessageRateAndRetry(RecomendedWaitTime<'a>),

    /// Error [FcmResponseError::Unavailable] or [FcmResponseError::Internal]
    /// was detected. Check [RecomendedWaitTime] to determine
    /// should specific or exponential back-off wait time should be used as
    /// a waiting time. After the waiting time is elapsed then resend the
    /// previous message.
    Retry(RecomendedWaitTime<'a>),

    /// Error [FcmResponseError::ThirdPartyAuth] was detected. Check
    /// credentials related to iOS and web push notifications.
    CheckIosAndWebCredentials,

    /// Error [FcmResponseError::Unspecified] or [FcmResponseError::Unknown]
    /// was detected. It is not clear what to do to handle this case.
    HandleUnknownError,
}

impl RecomendedAction<'_> {
    fn analyze(response: &FcmResponse) -> Option<RecomendedAction> {
        let action = match response.error()? {
            FcmResponseError::Unspecified | FcmResponseError::Unknown { .. } => RecomendedAction::HandleUnknownError,
            FcmResponseError::Unregistered => RecomendedAction::RemoveFcmAppToken,
            FcmResponseError::InvalidArgument => RecomendedAction::FixMessageContent,
            FcmResponseError::SenderIdMismatch => RecomendedAction::CheckSenderIdEquality,
            FcmResponseError::QuotaExceeded => {
                let wait_time = if let Some(ra) = response.retry_after() {
                    RecomendedWaitTime::SpecificWaitTime(ra)
                } else {
                    RecomendedWaitTime::InitialWaitTime(Duration::from_secs(60))
                };

                RecomendedAction::ReduceMessageRateAndRetry(wait_time)
            }
            FcmResponseError::Unavailable => {
                let wait_time = if let Some(ra) = response.retry_after() {
                    RecomendedWaitTime::SpecificWaitTime(ra)
                } else {
                    RecomendedWaitTime::InitialWaitTime(Duration::from_secs(10))
                };

                RecomendedAction::Retry(wait_time)
            }
            FcmResponseError::Internal => {
                RecomendedAction::Retry(RecomendedWaitTime::InitialWaitTime(Duration::from_secs(10)))
            }
            FcmResponseError::ThirdPartyAuth => RecomendedAction::CheckIosAndWebCredentials,
        };
        Some(action)
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
        assert_eq!(
            expected_wait_time,
            expected.wait_time_with_time_provider(DateTime::default)
        );
    }

    #[test]
    fn test_retry_after_from_date() {
        let date = "Sun, 06 Nov 1994 08:49:37 GMT";
        let date_time = DateTime::parse_from_rfc2822(date).unwrap();
        let retry_after = RetryAfter::from_str(date).unwrap();

        assert_eq!(RetryAfter::DateTime(date_time), retry_after,);

        assert_eq!(Duration::ZERO, retry_after.wait_time_with_time_provider(|| date_time),);
    }

    #[test]
    fn test_retry_after_from_date_and_get_wait_time_using_future_date() {
        let date = "Sun, 06 Nov 1994 08:49:37 GMT";
        let retry_after = RetryAfter::from_str(date).unwrap();
        let future_date = "Sun, 06 Nov 1994 08:49:38 GMT";
        let future_date_time = DateTime::parse_from_rfc2822(future_date).unwrap();

        assert_eq!(
            Duration::from_secs(0),
            retry_after.wait_time_with_time_provider(|| future_date_time),
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
            retry_after.wait_time_with_time_provider(|| past_date_time),
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
            retry_after.wait_time_with_time_provider(|| past_date_time),
        );
    }
}

// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for `Status`.
//  The parsing should be case-insensitive.

#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TryFrom<String> for Status {
    type Error = ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match &value.to_lowercase()[..] {
            "todo" => Ok(Status::ToDo),
            "inprogress" => Ok(Status::InProgress),
            "done" => Ok(Status::Done),
            other => Err(ParseError{invalid_status:other.into()})
        }
    }
}

impl TryFrom<&str> for Status {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "todo" => Ok(Status::ToDo),
            "inprogress" => Ok(Status::InProgress),
            "done" => Ok(Status::Done),
            other => Err(ParseError{invalid_status:other.into()})
        }
    }
}

#[derive(Debug, PartialEq, thiserror::Error)]
#[error("{invalid_status} is not valid")]
struct ParseError {
    invalid_status: String
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let status = Status::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("todo").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inprogress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("done").unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_str_error() {
        let err = Status::try_from("td").unwrap_err();
        assert_eq!(err, ParseError{invalid_status:"td".to_string()});
    }
}

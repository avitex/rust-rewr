use dangerous::error::{ExpectedValid, ToRetryRequirement, WithContext};
use dangerous::{Input, Reader};

// pub fn alt_peek_token() {}

pub fn alt<'i, T, I, E>(
    r: &mut Reader<'i, I, E>,
    expected: &'static str,
    choices: impl Choices<'i, T, I, E>,
) -> Result<T, E>
where
    I: Input<'i>,
    E: WithContext<'i>,
    E: From<ExpectedValid<'i>>,
{
    r.expect(expected, |r| alt_opt(r, choices))
}

pub fn try_alt<'i, T, I, E>(
    r: &mut Reader<'i, I, E>,
    expected: &'static str,
    choices: impl TryChoices<'i, T, I, E>,
) -> Result<T, E>
where
    I: Input<'i>,
    E: WithContext<'i>,
    E: From<ExpectedValid<'i>>,
{
    r.try_expect(expected, |r| try_alt_opt(r, choices))
}

pub fn alt_opt<'i, T, I, E>(
    r: &mut Reader<'i, I, E>,
    choices: impl Choices<'i, T, I, E>,
) -> Option<T> {
    choices.choice(r)
}

pub fn try_alt_opt<'i, T, I, E>(
    r: &mut Reader<'i, I, E>,
    choices: impl TryChoices<'i, T, I, E>,
) -> Result<Option<T>, E> {
    choices.try_choice(r)
}

pub trait Choices<'i, T, I, E> {
    fn choice(self, r: &mut Reader<'i, I, E>) -> Option<T>;
}

impl<'i, T, I, E> Choices<'i, T, I, E> for &[fn(&mut Reader<'i, I, E>) -> Option<T>] {
    #[inline]
    fn choice(self, r: &mut Reader<'i, I, E>) -> Option<T> {
        for branch in self {
            if let Some(value) = (branch)(r) {
                return Some(value);
            }
        }
        None
    }
}

impl<'i, T, I, E, const N: usize> Choices<'i, T, I, E>
    for [fn(&mut Reader<'i, I, E>) -> Option<T>; N]
{
    fn choice(self, r: &mut Reader<'i, I, E>) -> Option<T> {
        self.as_ref().choice(r)
    }
}

pub trait TryChoices<'i, T, I, E> {
    fn try_choice(self, r: &mut Reader<'i, I, E>) -> Result<Option<T>, E>;
}

impl<'i, T, I, E, const N: usize> TryChoices<'i, T, I, E>
    for [fn(&mut Reader<'i, I, E>) -> Result<T, E>; N]
where
    I: Input<'i>,
    E: WithContext<'i>,
    E: ToRetryRequirement,
{
    fn try_choice(self, r: &mut Reader<'i, I, E>) -> Result<Option<T>, E> {
        self.as_ref().try_choice(r)
    }
}

impl<'i, T, I, E> TryChoices<'i, T, I, E> for &[fn(&mut Reader<'i, I, E>) -> Result<T, E>]
where
    I: Input<'i>,
    E: WithContext<'i>,
    E: ToRetryRequirement,
{
    #[inline]
    fn try_choice(self, r: &mut Reader<'i, I, E>) -> Result<Option<T>, E> {
        for branch in self {
            if let Some(value) = r.recover_if(branch, ToRetryRequirement::is_fatal)? {
                return Ok(Some(value));
            }
        }
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use dangerous::error::{Invalid, RetryRequirement};
    use dangerous::{BytesReader, Error, Input};

    fn try_alt_parse<'i, E>(r: &mut BytesReader<'i, E>) -> Result<(), E>
    where
        E: Error<'i>,
    {
        fn consume_hello<'i, E>(r: &mut BytesReader<'i, E>) -> Result<(), E>
        where
            E: Error<'i>,
        {
            r.consume("hello")
        }

        fn consume_world<'i, E>(r: &mut BytesReader<'i, E>) -> Result<(), E>
        where
            E: Error<'i>,
        {
            r.consume("world")
        }

        try_alt(r, "either hello or world", [consume_hello, consume_world])
    }

    #[test]
    fn test_try_alt() {
        assert_eq!(
            dangerous::input(b"hello").read_all(try_alt_parse),
            Ok::<_, Invalid>(())
        );

        assert_eq!(
            dangerous::input(b"world").read_all(try_alt_parse),
            Ok::<_, Invalid>(())
        );

        assert_eq!(
            dangerous::input(b"foobar").read_all(try_alt_parse),
            Err::<_, Invalid>(Invalid::fatal())
        );

        assert_eq!(
            dangerous::input(b"hel").read_all(try_alt_parse),
            Err::<_, Invalid>(Invalid::from(RetryRequirement::new(2)))
        );
    }
}

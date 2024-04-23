pub mod error {
    use kira::manager::backend::cpal::Error;
    use kira::manager::error::PlaySoundError;
    use kira::sound::FromFileError;
    use std::fmt::{Display, Formatter};

    /// The one and only error returned by this library
    #[derive(Debug)]
    pub enum OxAgAudioToolError {
        /// Audio manager issue, see the wrapped [Error]
        AudioManagerError(Error),
        /// File issue, probably a path to an audio file wasn't correct, see the wrapped [FromFileError]
        FileError(FromFileError),
        /// Audio playing issue, see the wrapper [PlaySoundError]
        PlaySoundError(PlaySoundError<()>),
    }

    impl Display for OxAgAudioToolError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let error_string = match self {
                OxAgAudioToolError::AudioManagerError(e) => format!("{}", e),
                OxAgAudioToolError::FileError(e) => format!("{}", e),
                OxAgAudioToolError::PlaySoundError(e) => format!("{}", e),
            };

            write!(f, "{}", error_string)
        }
    }

    impl From<Error> for OxAgAudioToolError {
        fn from(value: Error) -> Self {
            OxAgAudioToolError::AudioManagerError(value)
        }
    }

    impl From<FromFileError> for OxAgAudioToolError {
        fn from(value: FromFileError) -> Self {
            OxAgAudioToolError::FileError(value)
        }
    }

    impl From<PlaySoundError<()>> for OxAgAudioToolError {
        fn from(value: PlaySoundError<()>) -> Self {
            OxAgAudioToolError::PlaySoundError(value)
        }
    }
}

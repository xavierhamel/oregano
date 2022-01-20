use thiserror::Error;

pub type Error = Box<dyn std::error::Error>;

#[derive(Error, Debug)]
pub enum Dom {
    #[error("The queried element `{0}` was not found in the current page.")]
    ElementNotFound(String),

    #[error("The value of the input cannot be casted to `{0}`.")]
    InputType(String),

    #[error("The conversion of an element was not successful.")]
    ConvertElement,

    #[error("An error occured.")]
    Other,
}

#[derive(Error, Debug)]
pub enum Import {
    #[error("The imported file is corrupted.")]
    MissingToken,
    #[error("The imported file is corrupted.")]
    UnexpectedValue,
}

#[derive(Error, Debug)]
pub enum Internal {
    #[error("An internal error occured.")]
    Event,
    #[error("An internal error occured.")]
    Parse,
}

#[derive(Error, Debug)]
pub enum Sim {
    #[error("Un composant doit avoir une propriétée nommée `{1}` mais la propriétée n'a pas été trouvée")]
    PropertyNotFound(usize, String),
    #[error("Le nom d'un composant doit être au moins de 1 caractère")]
    EmptyName,
    #[error("Le composant n'est pas connecté à toutes ses bornes")]
    MissingConnection(usize, usize),
    #[error("Le même node est nommé plusieurs fois.")]
    MultipleNameOnNode(usize),
    #[error("Le circuit comporte aucun ground.")]
    NoGround,
    #[error("Aucune probe est présent dans le circuit.")]
    NoProbe,
}

use crate::dom;
use thiserror::Error;

pub type Error = Box<dyn std::error::Error>;

pub fn show(error: Error) {
    let container = dom::select("#error__messages");
    container.set_inner_html("");
    dom::append_children(&container, vec![&to_html(&error)]);
    let _ = dom::select("#error__container").set_attribute("class", "");
}

pub fn show_multiple(errors: Vec<Error>) {
    let container = dom::select("#error__messages");
    container.set_inner_html("");
    errors
        .iter()
        .for_each(|error| dom::append_children(&container, vec![&to_html(error)]));
    let _ = dom::select("#error__container").set_attribute("class", "");
}

fn to_html(error: &Error) -> web_sys::Element {
    let message = error.to_string();
    dom::create_element(
        "div",
        dom::attributes! {
            "class" => "error__message",
            "inner_html" => &message,
        },
        vec![],
    )
}

#[derive(Error, Debug)]
pub enum Dom {
    #[error("The queried element `{0}` was not found in the current page.")]
    ElementNotFound(String),
    #[error("The value of the input cannot be casted to `{0}`.")]
    InputType(String),
    #[error("The conversion of an element was not successful.")]
    ConvertElement,
}

#[derive(Error, Debug)]
pub enum Import {
    #[error("The imported file is corrupted.")]
    MissingToken,
    #[error("The imported file is corrupted.")]
    UnexpectedValue,
    #[error("The part was not found.")]
    PartNotFound,
    #[error("The library was not found.")]
    LibNotFound,
}

#[derive(Error, Debug)]
pub enum Internal {
    #[error("An internal error occured.")]
    Event,
    #[error("An internal error occured.")]
    Parse,
    #[error("An internal error occured.")]
    Probe,
}

#[derive(Error, Debug)]
pub enum Sim {
    #[error("Un composant doit avoir une propriétée nommée `{1}` mais la propriétée n'a pas été trouvée")]
    PropertyNotFound(usize, String),
    #[error("Le nom d'un composant doit être au moins de 1 caractère")]
    EmptyName,
    #[error("Un des connecteurs d'un composant n'est pas connecté.")]
    MissingConnectionPart(usize),
    #[error("Un des noeuds est en circuit ouvert. (`{0}`)")]
    MissingConnectionNode(String),
    #[error("Le même node est nommé plusieurs fois.")]
    MultipleNameOnNode(usize),
    #[error("Le circuit comporte aucun ground.")]
    NoGround,
    #[error("Aucune probe est présent dans le circuit.")]
    NoProbe,
    #[error("Plusieurs composants ont le même nom : `{0}`")]
    MultipleSameName(String),
    #[error("Le nom du ground devrait toujours être `0`")]
    GroundWithBadName,
    #[error("Une erreur est survenue pendant la simulation, vérifier votre circuit.")]
    SpiceNoData,
    #[error("L'analyse sélectionné n'existe pas, veuillez en sélectionner une autre")]
    UnavailableAnalysis,
}

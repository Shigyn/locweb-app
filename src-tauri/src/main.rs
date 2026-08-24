// ===================================================================
//  Mon espace LocWeb — application de bureau (Windows et Linux).
//
//  L'application n'embarque pas l'espace client : elle ouvre une
//  fenetre sur https://admin.locweb.fr. C'est volontaire. L'espace est
//  corrige et redeploye plusieurs fois par semaine ; une copie figee
//  dans un installeur serait perimee des le lendemain, et il faudrait
//  faire retelecharger l'application a chaque correction.
//
//  Ce que l'application apporte par rapport a l'onglet du navigateur :
//  une icone dans le menu Demarrer, une fenetre sans barre d'adresse,
//  et une session qui reste ouverte d'un lancement a l'autre.
// ===================================================================

// Sans ceci, lancer l'application sous Windows ouvre aussi une console
// noire derriere la fenetre.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_opener::OpenerExt;

const ACCUEIL: &str = "https://admin.locweb.fr/";

/// Les domaines qui ont le droit de s'afficher DANS la fenetre.
///
/// La liste est courte exprès. Tout le reste part vers le navigateur du
/// client : un lien vers son propre site, ouvert dans la fenetre de
/// l'application, remplacerait l'espace par une page sans barre
/// d'adresse ni bouton retour — le client serait coince.
fn interne(hote: &str) -> bool {
    hote == "admin.locweb.fr"
        // Le parcours de connexion Google passe par plusieurs domaines
        // avant de revenir, et Supabase recoit la redirection finale.
        || hote == "accounts.google.com"
        || hote.ends_with(".google.com")
        || hote.ends_with(".supabase.co")
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let poignee = app.handle().clone();

            WebviewWindowBuilder::new(
                app,
                "principale",
                WebviewUrl::External(ACCUEIL.parse().expect("adresse d'accueil invalide")),
            )
            .title("Mon espace LocWeb")
            .inner_size(1180.0, 820.0)
            // Le seuil bas correspond a la mise en page telephone de
            // l'espace : en dessous, les tableaux debordent.
            .min_inner_size(380.0, 560.0)
            .center()
            .on_navigation(move |url| {
                let hote = url.host_str().unwrap_or("");
                if interne(hote) {
                    return true;
                }
                // Un echec ici ne doit pas figer l'application : au pire
                // le lien ne s'ouvre pas, la fenetre reste utilisable.
                let _ = poignee.opener().open_url(url.as_str(), None::<&str>);
                false
            })
            .build()?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("impossible de demarrer l'application");
}

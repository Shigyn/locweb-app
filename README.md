# Mon espace LocWeb — les applications installables

Ce dépôt ne contient pas l'espace client. Il contient les **coquilles**
qui l'installent sur un appareil. L'espace lui-même vit dans
`locweb-editeur` et se déploie sur `admin.locweb.fr`.

C'est volontaire : l'espace est corrigé plusieurs fois par semaine. Une
copie figée dans un installeur serait périmée dès le lendemain, et il
faudrait faire retélécharger l'application à chaque correction. Ici,
l'application ouvre une fenêtre sur `admin.locweb.fr` — un client
installe une fois, et reçoit toutes les corrections sans rien faire.

| Système | Fichier livré | Technique |
|---|---|---|
| Windows | `LocWeb-Espace-Windows.exe` | Tauri (WebView2) |
| Linux | `LocWeb-Espace-Linux.AppImage` et `.deb` | Tauri (WebKitGTK) |
| Android | `LocWeb-Espace-Android.apk` | TWA (Bubblewrap) |
| iPhone, iPad | — | Safari, « Sur l'écran d'accueil » |

## Publier une nouvelle version

```bash
git tag v1.0.1
git push origin v1.0.1
```

C'est tout. Les deux workflows construisent, renomment et publient les
fichiers dans une release GitHub. La page
`admin.locweb.fr/telecharger.html` pointe sur
`releases/latest/download/...` : ses liens n'ont jamais besoin d'être
mis à jour.

Pour vérifier une construction sans publier : onglet **Actions**, choisir
le workflow, **Run workflow**. Les fichiers sortent en artefacts.

## Ce qu'il faut savoir avant de diffuser

**Le programme Windows n'est pas signé.** À l'ouverture, Windows affiche
« Windows a protégé votre ordinateur » ; il faut cliquer sur
« Informations complémentaires » puis « Exécuter quand même ». Faire
disparaître cet écran demande un certificat de signature de code, autour
de 300 € par an. Tant qu'il n'est pas acheté, l'installation par le
navigateur — proposée en premier sur la page de téléchargement —
n'affiche aucun avertissement et donne exactement la même application.

**L'apk s'installe hors du Play Store.** Android demande alors
d'autoriser les installations depuis le navigateur, et Play Protect
affiche un avertissement. Sur Android, le bouton « Installer
l'application » de Chrome produit une vraie application, sans
avertissement ni réglage à changer : c'est le chemin à conseiller, l'apk
est là pour les cas où on le demande.

## La clé de signature Android

Elle est **hors du dépôt**, dans `../cles-android/`. Deux secrets à
créer dans *Settings → Secrets and variables → Actions* :

| Secret | Contenu |
|---|---|
| `CLE_ANDROID_BASE64` | le contenu de `cles-android/locweb.p12.base64` |
| `CLE_ANDROID_MDP` | le mot de passe noté dans `cles-android/mot-de-passe.txt` |

Son empreinte SHA-256 est publiée dans
`locweb-editeur/.well-known/assetlinks.json`. C'est ce couple
empreinte / clé qui fait disparaître la barre d'adresse Chrome dans
l'application Android. **Si les deux cessent de correspondre,
l'application fonctionne mais garde un bandeau en haut** : c'est le
symptôme à reconnaître.

Perdre la clé rend toute mise à jour impossible — Android refuse une
version signée par une autre clé. Sauvegardez `cles-android/` ailleurs
que sur le poste.

## Direction visuelle



Le concept du logo :

- un **cercle central** = le serveur / point d’accès,

- deux **nœuds latéraux** = appareils du réseau local,

- une **forme en L** intégrée = Laravel / Laralink,

- une géométrie simple = bonne lisibilité en favicon, tray icon ou header. [dev](https://dev.to/albert_nahas_cdc8469a6ae8/designing-icons-that-work-in-both-light-and-dark-mode-23d6)



Je te conseille aussi de ne pas mettre de dégradé dans le SVG principal. Les SVG/logo de produit restent plus robustes en light/dark mode avec `currentColor` et quelques variables de surface, plutôt qu’avec des couleurs fixes ou une inversion automatique. [paletterx](https://paletterx.com/blog/color-in-svg-icons)



## Palette recommandée



Je te propose une palette sobre, orientée produit desktop technique, avec une dominante teal/vert bleuté. Les systèmes de design modernes recommandent des tokens de couleurs et des variantes explicites pour le mode sombre plutôt qu’une simple inversion visuelle. [thefrontkit](https://thefrontkit.com/docs/finance-dashboard-kit/design-tokens)



### Mode clair

- `--brand-bg: #f7f6f2`

- `--brand-surface: #f9f8f5`

- `--brand-text: #28251d`

- `--brand-muted: #7a7974`

- `--brand-primary: #01696f`

- `--brand-primary-soft: #cedcd8`

- `--brand-border: #d4d1ca` [maviklabs](https://www.maviklabs.com/blog/design-tokens-tailwind-v4-2026/)



### Mode sombre

- `--brand-bg: #171614`

- `--brand-surface: #1c1b19`

- `--brand-text: #cdccca`

- `--brand-muted: #797876`

- `--brand-primary: #4f98a3`

- `--brand-primary-soft: #313b3b`

- `--brand-border: #393836` [deepwiki](https://deepwiki.com/tyrchen/rust-tauri-template/3.3-styling-system)



### Règles d’usage

- Le logo principal doit hériter de `color: var(--brand-text)` si tu veux une version monochrome.

- Pour la version accentuée, le contour ou le nœud central peut utiliser `var(--brand-primary)`.

- Les éléments SVG doivent utiliser `fill="currentColor"` ou `stroke="currentColor"` afin de suivre automatiquement le thème. [allsvgicons](https://allsvgicons.com/blog/change-svg-icon-color-with-css/)



## SVG proposé



Voici un premier logo SVG pour **Laralink**. Il est volontairement simple, propre, et prêt à être intégré inline dans Tauri/Vue.



```svg

<svg

  width="128"

  height="128"

  viewBox="0 0 128 128"

  fill="none"

  xmlns="http://www.w3.org/2000/svg"

  role="img"

  aria-label="Laralink logo"

>

  <defs>

    <style>

      .mark-stroke {

        stroke: currentColor;

        stroke-width: 8;

        stroke-linecap: round;

        stroke-linejoin: round;

      }

      .mark-fill {

        fill: currentColor;

      }

      .accent {

        fill: var(--logo-accent, currentColor);

      }

      .accent-stroke {

        stroke: var(--logo-accent, currentColor);

        stroke-width: 8;

        stroke-linecap: round;

        stroke-linejoin: round;

      }

    </style>

  </defs>



  <!-- Ring -->

  <circle cx="64" cy="64" r="42" class="mark-stroke" opacity="0.18"/>



  <!-- Network branches -->

  <path d="M64 34V64" class="accent-stroke"/>

  <path d="M64 64L38 80" class="mark-stroke"/>

  <path d="M64 64L90 80" class="mark-stroke"/>



  <!-- Stylized L -->

  <path d="M50 42V82C50 88 54 92 60 92H82" class="accent-stroke"/>



  <!-- Nodes -->

  <circle cx="64" cy="64" r="9" class="accent"/>

  <circle cx="38" cy="80" r="7" class="mark-fill"/>

  <circle cx="90" cy="80" r="7" class="mark-fill"/>

  <circle cx="64" cy="34" r="7" class="mark-fill"/>

</svg>

```



Ce logo utilise `currentColor` pour la majorité des formes et une variable `--logo-accent` pour la couleur principale. C’est la méthode la plus souple pour assurer une adaptation propre au light/dark mode sans multiplier les fichiers SVG. [syire](https://syire.com/en/posts/logo-png-to-currentcolor-svg/)



## Exemple d’intégration CSS



Voici comment l’utiliser dans ton app :



```css

:root,

[data-theme="light"] {

  --brand-bg: #f7f6f2;

  --brand-surface: #f9f8f5;

  --brand-text: #28251d;

  --brand-muted: #7a7974;

  --brand-primary: #01696f;

  --brand-primary-soft: #cedcd8;

  --brand-border: #d4d1ca;

}



[data-theme="dark"] {

  --brand-bg: #171614;

  --brand-surface: #1c1b19;

  --brand-text: #cdccca;

  --brand-muted: #797876;

  --brand-primary: #4f98a3;

  --brand-primary-soft: #313b3b;

  --brand-border: #393836;

}



.logo {

  color: var(--brand-text);

  --logo-accent: var(--brand-primary);

}



.logo--muted {

  color: var(--brand-muted);

  --logo-accent: var(--brand-primary);

}



.logo--mono {

  color: var(--brand-text);

  --logo-accent: currentColor;

}

```



Avec cette approche, le même SVG peut servir dans le header, la page splash, le tray menu ou un bouton settings, tout en restant cohérent avec le thème choisi. Les bonnes pratiques actuelles pour les SVG adaptatifs recommandent justement ce modèle basé sur `currentColor` et variables CSS. [echobind](https://echobind.com/post/currentcolor-css-property-with-svg)



## Variantes conseillées



Je te recommande de prévoir 3 variantes officielles :



- **Logo principal** : texte sombre + accent teal.

- **Logo monochrome** : tout en `currentColor`, pour favicon, tray ou bouton compact.

- **Logo inversé contextuel** : même SVG, mais dans un conteneur où `color` change selon le fond. [axdn](https://axdn.com/learn/themeable-icons-currentcolor-dark-mode)



Tu peux aussi définir ces usages :

- Header app : `color = brand-text`, accent = `brand-primary`.

- Carte active : `color = brand-text`, accent = `brand-primary`.

- Bouton secondaire / toolbar : `color = brand-muted`, accent = `brand-primary`.

- Splash screen : logo centré sur fond `brand-bg`. [thefrontkit](https://thefrontkit.com/docs/finance-dashboard-kit/design-tokens)



## Mini charte visuelle



Je te propose cette mini charte pour Laralink :



| Élément | Mode clair | Mode sombre |

|---|---|---|

| Fond principal | `#f7f6f2`  [thefrontkit](https://thefrontkit.com/docs/finance-dashboard-kit/design-tokens) | `#171614`  [thefrontkit](https://thefrontkit.com/docs/finance-dashboard-kit/design-tokens) |

| Surface | `#f9f8f5`  [thefrontkit](https://thefrontkit.com/docs/finance-dashboard-kit/design-tokens) | `#1c1b19`  [thefrontkit](https://thefrontkit.com/docs/finance-dashboard-kit/design-tokens) |

| Texte principal | `#28251d`  [thefrontkit](https://thefrontkit.com/docs/finance-dashboard-kit/design-tokens) | `#cdccca`  [thefrontkit](https://thefrontkit.com/docs/finance-dashboard-kit/design-tokens) |

| Texte secondaire | `#7a7974`  [thefrontkit](https://thefrontkit.com/docs/finance-dashboard-kit/design-tokens) | `#797876`  [thefrontkit](https://thefrontkit.com/docs/finance-dashboard-kit/design-tokens) |

| Accent marque | `#01696f`  [thefrontkit](https://thefrontkit.com/docs/finance-dashboard-kit/design-tokens) | `#4f98a3`  [thefrontkit](https://thefrontkit.com/docs/finance-dashboard-kit/design-tokens) |

| Bordure | `#d4d1ca`  [thefrontkit](https://thefrontkit.com/docs/finance-dashboard-kit/design-tokens) | `#393836`  [thefrontkit](https://thefrontkit.com/docs/finance-dashboard-kit/design-tokens) |



Cette palette reste discrète, technique, et cohérente avec un produit Tauri utilitaire plutôt qu’une landing page marketing. [maviklabs](https://www.maviklabs.com/blog/design-tokens-tailwind-v4-2026/)



## Recommandation finale



Je te conseille de retenir :

- **Nom** : Laralink

- **Style** : géométrique, minimal, technique

- **Accent principal** : teal

- **SVG inline** avec `currentColor`

- **Pas de dégradé** sur le logo de base

- **Un seul logo source** piloté par variables CSS en light/dark mode. [paletterx](https://paletterx.com/blog/color-in-svg-icons)


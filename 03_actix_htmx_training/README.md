# Rust and HTMX | Postmodern WebDev | CodeScope
- https://youtu.be/qCXFi4Jg11c?si=CyyksUe45xS3W8sD

- https://gitlab.com/codescope-reference/rustmx

- index.html
  - https://github.com/bigskysoftware/contact-app

# Rust코드가 아니지만 변환해서 쓰면 될듯(htmx 예시)
- https://github.com/bigskysoftware/contact-app


# Web개발의 역사

|Old school|"Raw", HTML, JS, CSS|
|-|-|
|Modern| React, Vue, Svelte, Angular|
|PostModern| HTMX|
|WASM| C+Raylib, Rust, etc|

- HTMX
  - Requirements:
    - Web server
      - -> Rust with actix-web
    - HTML templating engine to enable components and HTML code reuse
      - -> tera
    - Basic styling
      - -> missing.style

# rust tera
- https://github.com/Keats/tera

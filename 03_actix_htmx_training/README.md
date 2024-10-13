# Rust and HTMX | Postmodern WebDev | CodeScope(part1)
- https://youtu.be/qCXFi4Jg11c?si=CyyksUe45xS3W8sD

- https://gitlab.com/codescope-reference/rustmx

- index.html
  - https://github.com/bigskysoftware/contact-app

- (part1) Rust and HTMX | Postmodern WebDev | CodeScope
  - https://youtu.be/qCXFi4Jg11c?si=CyyksUe45xS3W8sD
    - (part2) Adding a database to the Rust+HTMX app | CodeScope
      - https://youtu.be/lu-KkjgMnCI?si=yKpaLJt3QKdaCKSy
    - (part3) Auth is a beast | Overview and password hashing | Rust + HTMX | CodeScope
      - https://youtu.be/OPZcT9P7Nls?si=_57YuQCKTpn8ArQJ
    - (part4) Auth is a beast | Secure tokens | Rust + HTMX | CodeScope
      - https://youtu.be/muFOblL0jJM?si=MLBg6W0a9ImcHJ3D
    - (part5) Auth is a beast | Login flow | Rust + HTMX | CodeScope
      - https://youtu.be/VVy-8e3JGQI?si=c8-CzWrDmYyl7LIc
    - (part6) Auth is a beast | MIddleware for login verification | Rust + HTMX | CodeScope
      - https://youtu.be/jROC-lK3jIM?si=iZxOqvIyK3qVUweh
    - (part7) Auth is a beast | Registration and password reset | Rust + HTMX | CodeScope
      - https://youtu.be/s3_rzFICKes?si=rQhMOtAut2zXbeEO 
 

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

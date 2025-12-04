# lion

## About

This is the repository for my personal [website](https://robin-thoene.com/).

## Local development

Ensure to have installed:

- npm
- [watchexec](https://github.com/watchexec/watchexec)

Start the Rust server

```shell
watchexec -r -- cargo run
```

Start the watch process for Tailwind CSS

```shell
npm i
npx @tailwindcss/cli -i ./styles/main.css -o ./static/css/main.css --watch
```

## Technology

The following technology choices have been made:

- [Axum](https://github.com/tokio-rs/axum) for serving HTTP requests
- [Askama](https://github.com/rinja-rs/askama) for rendering HTML templates
- [Tailwind CSS](https://tailwindcss.com/) to apply styles
- [Font Awesome](https://fontawesome.com/) as free core and brands icon set

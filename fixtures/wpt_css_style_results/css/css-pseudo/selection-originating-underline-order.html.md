# css/css-pseudo/selection-originating-underline-order.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/selection-originating-underline-order.html"
}
```

## style[0]

```css

    * {
        text-decoration-skip-ink: none;
        text-decoration-skip: none;
    }
    main {
        font: 80px/1 Ahem;
        margin: 0.5em;
        width: min-content;
        text-decoration: 0.25em red solid underline;
        /* try to keep decoration between ascent and descent */
        text-underline-offset: -0.5em;
    }
    main::selection {
        background: transparent;
        color: blue;
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

# css/css-pseudo/selection-originating-underline-order-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/selection-originating-underline-order-ref.html"
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
        /*
            Edge on Windows anti-aliases Ahem for some reason, which
            allows the tip of the test underline to show through, so
            we still need this underline despite seeming unnecessary.
        */
        text-decoration: 0.25em currentColor solid underline;
        /* try to keep decoration between ascent and descent */
        text-underline-offset: -0.5em;
    }
    main > span {
        color: blue;
        /*
            Edge on Windows anti-aliases Ahem for some reason, which
            allows the tip of the test underline to show through, so
            we still need this underline despite seeming unnecessary.
        */
        text-decoration: 0.25em blue solid underline;
        /* try to keep decoration between ascent and descent */
        text-underline-offset: -0.5em;
    }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “text-decoration”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “text-decoration”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

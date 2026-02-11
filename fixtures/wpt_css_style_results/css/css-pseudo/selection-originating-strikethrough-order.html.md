# css/css-pseudo/selection-originating-strikethrough-order.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/selection-originating-strikethrough-order.html"
}
```

## style[0]

```css

    main {
        font: 80px/1 Ahem;
        margin: 0.5em;
        width: min-content;
        color: black;
        text-decoration: 1em black solid line-through;
    }
    main::selection {
        /*
            Safari on macOS: with an opaque selection background, the
            used value will be translucent. The ref shouldn’t need to
            take this into account, as the selected text and its
            decorations should entirely obscure the background.
        */
        background: white;
        color: black;
    }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “text-decoration”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

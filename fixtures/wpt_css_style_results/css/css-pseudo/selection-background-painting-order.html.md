# css/css-pseudo/selection-background-painting-order.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/selection-background-painting-order.html"
}
```

## style[0]

```css

    @font-face {
        font-family: CSSTest;
        src: url(/fonts/CSSTest/csstest-basic-italic.ttf);
    }
    main {
        font: 80px/1 CSSTest;
        margin: 0.5em;
        width: min-content;
    }
    main > span {
        background: black;
        color: orange;
    }
    main > span::selection {
        /*
            Safari on macOS: opaque selection backgrounds are made
            translucent in the used value, so we need to choose a
            background color with known but non-opaque alpha.
        */
        background: #0000FFC0;
        color: transparent;
    }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

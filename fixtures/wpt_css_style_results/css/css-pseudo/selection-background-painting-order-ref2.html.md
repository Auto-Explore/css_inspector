# css/css-pseudo/selection-background-painting-order-ref2.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/selection-background-painting-order-ref2.html"
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
    .unselected {
        background: black;
    }
    .orange {
        color: orange;
    }
    .selected {
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

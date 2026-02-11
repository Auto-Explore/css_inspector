# css/css-pseudo/selection-originating-decoration-color.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/selection-originating-decoration-color.html"
}
```

## style[0]

```css

    main {
        font: 80px/1 Ahem;
        margin: 0.5em;
        width: min-content;
        color: black;
        text-decoration: 0.125em black solid line-through;
    }
    main::selection {
        background: transparent;
        color: green;
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

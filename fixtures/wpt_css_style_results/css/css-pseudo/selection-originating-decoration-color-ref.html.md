# css/css-pseudo/selection-originating-decoration-color-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/selection-originating-decoration-color-ref.html"
}
```

## style[0]

```css

    main {
        font: 80px/1 Ahem;
        margin: 0.5em;
        width: min-content;
    }
    main .black {
        color: black;
        text-decoration: 0.125em black solid line-through;
    }
    main .green {
        color: green;
        text-decoration: 0.125em green solid line-through;
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

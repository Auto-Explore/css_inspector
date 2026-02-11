# css/css-pseudo/highlight-cascade/highlight-cascade-009.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/highlight-cascade/highlight-cascade-009.html"
}
```

## style[0]

```css

  body {
    --background-color: green;
    --decoration-color: green;
  }
  body::selection {
    --decoration-color: purple;
  }
  div::selection {
    --background-color: blue;
    background-color: var(--background-color, red);
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

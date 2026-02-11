# css/css-pseudo/highlight-cascade/highlight-cascade-011.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/highlight-cascade/highlight-cascade-011.html"
}
```

## style[0]

```css

  :root::selection {
    --background-color: red;
  }
  div::selection {
    background-color: var(--background-color, green);
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

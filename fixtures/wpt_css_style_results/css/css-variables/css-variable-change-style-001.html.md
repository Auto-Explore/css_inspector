# css/css-variables/css-variable-change-style-001.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/css-variable-change-style-001.html"
}
```

## style[0]

```css

    .inner {
        color: var(--x);
        background-color: var(--x);
        white-space: var(--x);
    }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

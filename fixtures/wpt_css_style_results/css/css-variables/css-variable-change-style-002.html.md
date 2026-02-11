# css/css-variables/css-variable-change-style-002.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/css-variable-change-style-002.html"
}
```

## style[0]

```css

    .test1 > div > div { color: var(--x); }
    .test2 > div > div { background-color: var(--x); }
    .test3 > div > div { white-space: var(--x); }
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

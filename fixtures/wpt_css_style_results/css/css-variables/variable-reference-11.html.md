# css/css-variables/variable-reference-11.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-reference-11.html"
}
```

## style[0]

```css

body {
  --a: green;
  color: red;
}
p {
  color: crimson;
  color: var(--a,/**/);
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
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

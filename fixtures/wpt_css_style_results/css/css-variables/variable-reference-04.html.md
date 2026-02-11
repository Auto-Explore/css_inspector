# css/css-variables/variable-reference-04.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-reference-04.html"
}
```

## style[0]

```css

:root {
  --a: green;
  color: red;
}
p {
  color: crimson;
  color: var(--a) var(--b, );
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

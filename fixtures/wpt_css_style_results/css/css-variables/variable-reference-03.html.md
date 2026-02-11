# css/css-variables/variable-reference-03.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-reference-03.html"
}
```

## style[0]

```css

:root {
  --a: green;
  --b: ;
  color: red;
}
p {
  color: crimson;
  color: var(--a) var(--b);
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Missing value for property “--b”.",
      "severity": "Error"
    },
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

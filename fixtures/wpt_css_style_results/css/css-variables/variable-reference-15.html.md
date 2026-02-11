# css/css-variables/variable-reference-15.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-reference-15.html"
}
```

## style[0]

```css

body {
  color: green;
}
p {
  color: crimson;
  --a: orange;
  --b: red;
  color: var(--a)var(--b);
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

# css/css-variables/variable-reference-08.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-reference-08.html"
}
```

## style[0]

```css

body {
  --a: crimson;
  color: red;
}
p {
  color: green;
  color: var(--a,!);
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

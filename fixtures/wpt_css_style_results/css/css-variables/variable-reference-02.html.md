# css/css-variables/variable-reference-02.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-reference-02.html"
}
```

## style[0]

```css

:root {
  --a: crimson;
  color: red;
}
body {
  color: green;
}
p {
  color: orange;
  color: var(--a) var(--b);
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

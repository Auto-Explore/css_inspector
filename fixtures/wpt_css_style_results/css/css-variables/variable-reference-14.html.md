# css/css-variables/variable-reference-14.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-reference-14.html"
}
```

## style[0]

```css

#a {
  --a: green !important;
}
p {
  color: red;
  --a: crimson;
  color: var(--a);
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

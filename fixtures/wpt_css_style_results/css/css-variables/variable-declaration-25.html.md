# css/css-variables/variable-declaration-25.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-declaration-25.html"
}
```

## style[0]

```css

body {
  color: green;
}
p {
  color: red;
}
p {
  color: orange;
  --a: red;
  --b: crimson;
  --a: --> var(--b); /* valid at parse */
  color: var(--a);   /* but IACVT at substitution */
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

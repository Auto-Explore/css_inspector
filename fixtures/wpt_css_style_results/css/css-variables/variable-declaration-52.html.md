# css/css-variables/variable-declaration-52.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-declaration-52.html"
}
```

## style[0]

```css

body {
  color: orange;
  --c: var(--a,green);
}
p {
  --a: var(--b);
}
p {
  color: red;
  --b: var(--c,crimson);
  color: var(--a);
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

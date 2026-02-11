# css/css-variables/variable-declaration-09.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-declaration-09.html"
}
```

## style[0]

```css

p {
  color: red;
  --a: crimson;
  --b: green;
  --a: var(--b,/**/);
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

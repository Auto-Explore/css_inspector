# css/css-variables/variable-declaration-55.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-declaration-55.html"
}
```

## style[0]

```css

p {
  color: green;
}
span {
  color: red;
  --a:var(--b)var(--c,red);
  --b:orange;
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

# css/css-variables/variable-declaration-18.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-declaration-18.html"
}
```

## style[0]

```css

body {
  font-family: serif;
}
p {
  font-family: monospace;
  --a: Ahem var(--b) sans-serif;
  --b: ,;
  font-family: var(--a);
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Missing value for property “--b”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

# css/css-fonts/system-ui-zh.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/system-ui-zh.html"
}
```

## style[0]

```css

@font-face {
  font-family: mplus;
  src: url(/fonts/mplus-1p-regular.woff);
  size-adjust: 50%; /* cause mismatch even if system-ui maps to M+ */
}
p {
  font-family: Ahem, system-ui, mplus, serif;
  font-size: 4em;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “size-adjust”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

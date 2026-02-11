# css/css-ui/appearance-auto-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-ui/appearance-auto-ref.html"
}
```

## style[0]

```css

 #container { width: 500px; }
 #container > :not(a) { appearance: auto; -webkit-appearance: auto; }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “-webkit-appearance”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

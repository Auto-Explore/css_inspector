# css/css-values/ric-invalidation.html

```json
{
  "format_version": 3,
  "file": "css/css-values/ric-invalidation.html"
}
```

## style[0]

```css

  @import url("/fonts/ahem.css");
  html {
    font-family: 'Ahem';
    font-size: 40px;
  }
  body {
    font-family: monospace;
    font-size: 20px;
  }
  div {
    width: 10ric;
  }
```

```json
{
  "errors": 0,
  "messages": [
    {
      "message": "Imported style sheets are not checked.",
      "severity": "Warning"
    }
  ],
  "warnings": 1
}
```

# css/css-align/abspos/align-self-stretch-auto-margins-aspect-ratio.html

```json
{
  "format_version": 3,
  "file": "css/css-align/abspos/align-self-stretch-auto-margins-aspect-ratio.html"
}
```

## style[0]

```css

.container {
  position: relative;
  width: 200px;
  height: 200px;
  top: -50px;
  left: -50px;
}
.abspos {
  position: absolute;
  inset: 50px auto 50px 50px;
  margin: auto 0 auto 0;
  align-self: stretch;
  background: green;
}
.abspos::before {
  content: '';
  min-width: 50px;
  height: 100%;
  aspect-ratio: 1;
  display: block;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “inset”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

# css/css-align/abspos/align-self-stretch-auto-margins.html

```json
{
  "format_version": 3,
  "file": "css/css-align/abspos/align-self-stretch-auto-margins.html"
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
  inset: 50px;
  margin: auto 0 auto 0;
  align-self: stretch;
  background: green;
}
.abspos::before {
  content: '';
  width: 50px;
  height: 50px;
  display: block;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

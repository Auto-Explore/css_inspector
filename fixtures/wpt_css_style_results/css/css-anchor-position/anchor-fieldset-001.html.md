# css/css-anchor-position/anchor-fieldset-001.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-fieldset-001.html"
}
```

## style[0]

```css

fieldset::before {
  content: "";
  display: block;
  width: 10px;
  height: 10px;
  background: purple;
  position: absolute;
  position-anchor: auto;
  position-area: bottom right;
}

fieldset {
  width: 200px;
  border: 10px solid blue;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

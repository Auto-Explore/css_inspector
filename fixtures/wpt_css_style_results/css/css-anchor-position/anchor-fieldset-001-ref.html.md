# css/css-anchor-position/anchor-fieldset-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-fieldset-001-ref.html"
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
  top: calc(100% + 10px);
  left: calc(100% + 10px);
}

fieldset {
  width: 200px;
  border: 10px solid blue;
  position: relative;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

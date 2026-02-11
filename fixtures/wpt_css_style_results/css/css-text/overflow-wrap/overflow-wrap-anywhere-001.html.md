# css/css-text/overflow-wrap/overflow-wrap-anywhere-001.html

```json
{
  "format_version": 3,
  "file": "css/css-text/overflow-wrap/overflow-wrap-anywhere-001.html"
}
```

## style[0]

```css

div {
  position: relative;
  width: 100px;
  height: 100px;
  font-family: Ahem;
  color: red;
  overflow-wrap: anywhere;
  font-size: 25px;
  line-height: 27px;
}
div::after{
  content: "";
  position: absolute;
  top: 0; left: 0; bottom: 0; right: 0;
  background: green;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

# css/css-text/overflow-wrap/overflow-wrap-anywhere-005.html

```json
{
  "format_version": 3,
  "file": "css/css-text/overflow-wrap/overflow-wrap-anywhere-005.html"
}
```

## style[0]

```css

div {
  position: relative;
  font-size: 20px;
  font-family: Ahem;
  line-height: 1em;
}
.fail {
  position: absolute;
  color: red;
  z-index: -1;
}
span { color: green; }
.test {
  color: green;
  width: 5ch;

  white-space: pre-wrap;
  overflow-wrap: anywhere;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

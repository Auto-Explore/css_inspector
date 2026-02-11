# css/css-text/white-space/break-spaces-with-overflow-wrap-010.html

```json
{
  "format_version": 3,
  "file": "css/css-text/white-space/break-spaces-with-overflow-wrap-010.html"
}
```

## style[0]

```css

div {
  font: 25px/1 Ahem;
}
.expected {
  position: absolute;
  color: green;
  width: 100px;
  height: 100px;
  white-space: pre;
}
.test {
  background: green;
  color: red;
  width: 4ch;

  white-space: break-spaces;
  overflow-wrap: anywhere;
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unbalanced braces.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

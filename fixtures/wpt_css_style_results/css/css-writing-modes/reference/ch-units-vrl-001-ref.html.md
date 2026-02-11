# css/css-writing-modes/reference/ch-units-vrl-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/reference/ch-units-vrl-001-ref.html"
}
```

## style[0]

```css

div {
  font-size: 20px;
  color: transparent;
}

div:nth-of-type(1) { background: green; }
div:nth-of-type(2) { background: blue; }
div:nth-of-type(1),
div:nth-of-type(2) {
  writing-mode: vertical-rl;
  text-orientation: upright;
  height: 5ch;
  width: 5ch;
}
div:nth-of-type(3) {
  background: orange;
  height: 5ch;
  width: 5ch;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

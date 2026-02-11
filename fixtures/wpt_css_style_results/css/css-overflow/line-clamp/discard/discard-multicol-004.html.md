# css/css-overflow/line-clamp/discard/discard-multicol-004.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/line-clamp/discard/discard-multicol-004.html"
}
```

## style[0]

```css

div {
  font-family: monospace;
  gap: 1ch;
  width: 27ch;
  columns: 3;
  height: 2lh;

  border: 1px solid;
  margin: 1em;
}
.test {
    continue: discard;
    block-overflow: no-ellipsis;
    max-lines: 5;
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “continue”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “block-overflow”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “max-lines”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

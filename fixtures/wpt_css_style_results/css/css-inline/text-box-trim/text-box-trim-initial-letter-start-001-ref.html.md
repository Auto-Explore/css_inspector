# css/css-inline/text-box-trim/text-box-trim-initial-letter-start-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-inline/text-box-trim/text-box-trim-initial-letter-start-001-ref.html"
}
```

## style[0]

```css

.spacer {
  height: 50px;
  background: lightgray;
}
.target {
  font: 40px/1 Ahem;
  position: relative;
}
.target::first-letter {
  initial-letter: 2 1;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “initial-letter”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

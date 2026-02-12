# css/css-inline/text-box-trim/text-box-trim-initial-letter-end-001.html

```json
{
  "format_version": 3,
  "file": "css/css-inline/text-box-trim/text-box-trim-initial-letter-end-001.html"
}
```

## style[0]

```css

.spacer {
  height: 50px;
  background: lightgray;
  position: relative;
}
.target {
  font: 40px/1 Ahem;
  text-box-trim: trim-end;
  text-box-edge: text alphabetic;
}
.target::first-letter {
  initial-letter: 3;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

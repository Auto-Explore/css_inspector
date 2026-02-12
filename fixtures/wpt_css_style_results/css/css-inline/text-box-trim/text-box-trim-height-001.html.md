# css/css-inline/text-box-trim/text-box-trim-height-001.html

```json
{
  "format_version": 3,
  "file": "css/css-inline/text-box-trim/text-box-trim-height-001.html"
}
```

## style[0]

```css

.spacer {
  background: lightgray;
  block-size: 20px;
}
.target {
  font: 100px/2 Ahem;
  text-box-trim: trim-both;
  text-box-edge: text alphabetic;
}

.height > .target {
  height: 120px;
}

.min-height > .target {
  min-height: 120px;
}

.max-height > .target {
  max-height: 10px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

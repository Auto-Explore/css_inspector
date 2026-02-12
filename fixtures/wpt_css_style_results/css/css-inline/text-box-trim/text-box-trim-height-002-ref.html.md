# css/css-inline/text-box-trim/text-box-trim-height-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-inline/text-box-trim/text-box-trim-height-002-ref.html"
}
```

## style[0]

```css

.spacer {
  background: lightgray;
  block-size: 20px;
}
.max-height > .spacer:first-child {
  margin-top: 80px; /* avoid overlap */
}

.target {
  font: 100px/1 Ahem;
}

.height > .target,
.min-height > .target {
  padding-top: 40px;
  height: 80px;
}

.max-height > .target {
  margin-top: -70px;
  height: 80px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

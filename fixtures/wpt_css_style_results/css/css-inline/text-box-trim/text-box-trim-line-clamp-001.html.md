# css/css-inline/text-box-trim/text-box-trim-line-clamp-001.html

```json
{
  "format_version": 3,
  "file": "css/css-inline/text-box-trim/text-box-trim-line-clamp-001.html"
}
```

## style[0]

```css

.spacer {
  background: lightgray;
  block-size: 100px;
}
.target {
  font: 50px/2 Ahem;
  text-box-trim: trim-end;
  text-box-edge: text;
}
.clamp {
  line-clamp: 3;
}
@supports not (line-clamp: 3) {
  .clamp {
    -webkit-line-clamp: 3;
    display: -webkit-box;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

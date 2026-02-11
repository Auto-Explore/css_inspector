# css/css-tables/caption-cyclic-percentage.html

```json
{
  "format_version": 3,
  "file": "css/css-tables/caption-cyclic-percentage.html"
}
```

## style[0]

```css

.test {
  display: inline-block;
  border: 10px solid;
}
.min-width > .test > div {
  min-width: calc(100px + 0%);
}
.width > .test > div {
  width: calc(100px + 0%);
}
.max-width > .test > div {
  max-width: calc(100px + 0%);
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

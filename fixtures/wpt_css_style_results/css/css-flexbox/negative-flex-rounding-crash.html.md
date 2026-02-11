# css/css-flexbox/negative-flex-rounding-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/negative-flex-rounding-crash.html"
}
```

## style[0]

```css

.math {
    display: inline-flex;
}
.math > * {
    padding-bottom: 0.35em;
}
.mfrac {
    display: inline-flex;
    flex-direction: column;
}
.mfrac > :first-child {
    margin-block-end: 0.2em;
}
.mfrac > :last-child {
    margin-block-start: 0.2em;
}
.x {
    line-height: 9px;
}
.y {
    line-height: 12px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

# css/css-sizing/contain-intrinsic-size/auto-008.html

```json
{
  "format_version": 3,
  "file": "css/css-sizing/contain-intrinsic-size/auto-008.html"
}
```

## style[0]

```css

.target {
  width: max-content;
  height: max-content;
  border: 1px solid;
}
.target::before {
  content: "";
  display: block;
  width: 100px;
  height: 50px;
}
.cis-auto .target {
  contain-intrinsic-size: auto 40px auto 20px;
}
.skip-contents {
  content-visibility: hidden;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

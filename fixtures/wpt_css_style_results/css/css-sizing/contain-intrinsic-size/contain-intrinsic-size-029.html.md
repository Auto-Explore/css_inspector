# css/css-sizing/contain-intrinsic-size/contain-intrinsic-size-029.html

```json
{
  "format_version": 3,
  "file": "css/css-sizing/contain-intrinsic-size/contain-intrinsic-size-029.html"
}
```

## style[0]

```css

.test {
  contain: size;
  padding: 0;
  border: 5px solid;
}
.test::before {
  content: '';
  display: block;
  width: 40px;
  height: 20px;
}
.cis-none {
  contain-intrinsic-size: none none;
}
.cis-height {
  contain-intrinsic-size: none 50px;
}
.cis-width {
  contain-intrinsic-size: 100px none;
}
.cis-both {
  contain-intrinsic-size: 100px 50px;
}
.min-size {
  min-width: 30px;
  min-height: 15px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

# css/css-sizing/contain-intrinsic-size/contain-intrinsic-size-030.html

```json
{
  "format_version": 3,
  "file": "css/css-sizing/contain-intrinsic-size/contain-intrinsic-size-030.html"
}
```

## style[0]

```css

.test {
  contain: size;
  display: inline-block;
  padding: 0;
  border: 5px solid;
}
.test::before {
  content: '';
  display: block;
  width: 40px;
  height: 20px;
}
.big-contents::before {
  width: 400px;
  height: 200px;
}
.overflow-auto {
  overflow: auto;
  scrollbar-gutter: stable;
}
.overflow-scroll {
  overflow: scroll;
}
.overflow-hidden {
  overflow: hidden;
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
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

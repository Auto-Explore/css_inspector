# css/css-images/gradient/gradient-to-transparent.html

```json
{
  "format_version": 3,
  "file": "css/css-images/gradient/gradient-to-transparent.html"
}
```

## style[0]

```css

.test {
  display: inline-block;
  width: 25px;
  height: 100px;
  margin: 10px;
}
.a {
  background: linear-gradient(transparent, 75%, red);
}
.b {
  background: linear-gradient(rgba(255 0 0 / 0), 75%, red);
}
.c {
  background: linear-gradient(rgba(0 0 255 / 0), 75%, red);
}
.d {
  background: linear-gradient(blue, 75%, transparent);
}
.e {
  background: linear-gradient(blue, 75%, rgba(0 0 255 / 0));
}
.f {
  background: linear-gradient(blue, 75%, rgba(0 255 0 / 0));
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

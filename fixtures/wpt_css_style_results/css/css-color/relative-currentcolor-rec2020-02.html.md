# css/css-color/relative-currentcolor-rec2020-02.html

```json
{
  "format_version": 3,
  "file": "css/css-color/relative-currentcolor-rec2020-02.html"
}
```

## style[0]

```css

div {
  width: 100px;
  height: 100px;
  background-color: red;
  color: red;
  background-color: color(from currentColor rec2020 r g b);
}
div div {
  color: color(prophoto-rgb 0.23 0.43 0.13); /* a little outside sRGB */
  background-color: inherit;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

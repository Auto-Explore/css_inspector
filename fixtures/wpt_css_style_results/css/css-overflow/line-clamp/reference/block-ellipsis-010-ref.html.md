# css/css-overflow/line-clamp/reference/block-ellipsis-010-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/line-clamp/reference/block-ellipsis-010-ref.html"
}
```

## style[0]

```css

@font-face {
  font-family: "Ahem";
  src: url("/fonts/Ahem.ttf");
  unicode-range: U+0000-002D, U+002F-007E;
}
@font-face {
  font-family: "Gentium Plus";
  src: url("/fonts/GentiumPlus-R.woff");
}

.container {
  display: grid;
  width: 500px;
  grid-template-columns: 50% 50%;
  align-items: end;
  grid-gap: 40px;
}

.box {
  background-color: pink;
  font-family: "Ahem", "Gentium Plus";
  font-size: 16px;
}
.ellipsis {
  line-height: 0;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

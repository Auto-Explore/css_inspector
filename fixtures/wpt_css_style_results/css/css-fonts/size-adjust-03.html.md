# css/css-fonts/size-adjust-03.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/size-adjust-03.html"
}
```

## style[0]

```css

@font-face {
  font-family: large-font;
  src: local(Ahem), url(/fonts/Ahem.ttf);
  size-adjust: 150%;
}

div {
  font-size: 40px;
  line-height: 100px;
}

.font-size-adjust-override {
  font-family: large-font, sans-serif;
  font-size-adjust: 0.8;
}

.descriptor-size-adjust-active {
  font-family: large-font, sans-serif;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

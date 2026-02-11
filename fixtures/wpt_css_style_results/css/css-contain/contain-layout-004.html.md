# css/css-contain/contain-layout-004.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-layout-004.html"
}
```

## style[0]

```css

div {
  position: relative;
  background: red;
  width: 100px;
  height: 100px;
  padding: 25px;
  box-sizing: border-box;
}
rtc {
  contain: layout;
  display: ruby-text-container;
  font-family: Ahem;
  font-size: 100px;
  line-height: 1;
}
rtc::after {
  content: "X";
  color: green;
  position: absolute;
  top:0; left: 0;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

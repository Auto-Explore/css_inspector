# css/css-contain/contain-layout-005.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-layout-005.html"
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
rt {
  contain: layout;
  display: ruby-text;
  font-family: Ahem;
  font-size: 100px;
  line-height: 1;
}
rt::after {
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

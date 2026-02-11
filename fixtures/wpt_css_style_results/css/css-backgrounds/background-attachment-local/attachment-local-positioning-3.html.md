# css/css-backgrounds/background-attachment-local/attachment-local-positioning-3.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/background-attachment-local/attachment-local-positioning-3.html"
}
```

## style[0]

```css

#outer {
  border: solid;
  /* 100% 100% == (250px - 32px) (370px - 32px) == 218px 338px */
  /* With scrolling, effective position is 178px 278px */
  background: url(aqua-yellow-32x32.png) local no-repeat 100% 100%;
  overflow: hidden;
  width: 200px;
  height: 300px;
}
div div {
  width: 250px;
  height: 370px;
}
p {
  margin: 0;
  padding-top: 100px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

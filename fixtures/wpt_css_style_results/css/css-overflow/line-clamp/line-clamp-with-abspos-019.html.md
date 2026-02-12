# css/css-overflow/line-clamp/line-clamp-with-abspos-019.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/line-clamp/line-clamp-with-abspos-019.html"
}
```

## style[0]

```css

#scrollContainer {
  overflow: scroll;
  position: relative;
  font: 16px / 32px serif;
  height: 4lh;
  border: 1px solid black;
}
.clamp {
  line-clamp: 4;
  padding: 0 4px;
  background-color: yellow;
}
.abspos {
  position: absolute;
  right: 0;
  width: 50px;
  height: 50px;
  margin: 4px;
  background-color: skyblue;
}
.rel {
  position: relative;
}
#abspos1 {
  top: 1lh;
}
#abspos2 {
  top: 2lh;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

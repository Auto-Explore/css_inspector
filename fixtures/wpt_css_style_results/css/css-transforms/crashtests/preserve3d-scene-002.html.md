# css/css-transforms/crashtests/preserve3d-scene-002.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/crashtests/preserve3d-scene-002.html"
}
```

## style[0]

```css


.outer {
  column-count:16384;
  -webkit-mask-box-image:url('?') 10% 90% repeat;
}
button {
  font:icon;
}
.inner {
  border-top:solid 1000000000px;
  transform-style:preserve-3d;
  display:list-item;
}
table {
  height:7.1px;
  padding-right:4em;
  transform:matrix3d(0,7.8,9,2,-3.6,3.4,6,8.8,9.5,7,7.3,5.3,6.4,3.7,8.3,8);
  border-left:outset black;
  font-size:120%;
}

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

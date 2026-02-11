# css/css-flexbox/flex-minimum-height-flex-items-015.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flex-minimum-height-flex-items-015.html"
}
```

## style[0]

```css

.overlapped-green {
  position: absolute;
  background-color: green;
  width: 100px;
  height: 80px;
  z-index: 1;
}
.outer {
  display: flex;
  width: 100px;
  background: green;
}
.inner {
  width: 100%;
  display: flex;
  flex-direction: column;
  min-width: 0;
}
.flex-item {
  height: 100%;
  margin-bottom: 20px;
  background: red;
}

.inside-of-item {
    height: 100%;
    width: 100px;
    min-height: 80px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

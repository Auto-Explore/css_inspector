# css/CSS2/zorder/z-index-020-ref.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/zorder/z-index-020-ref.xht"
}
```

## style[0]

```css

.container {
  z-index:0;
  position: relative;
  height: 200px;
  width: 200px;
  font-size: 0;
  line-height: 0;
  background: silver;
  border: solid white;
}
.container div {
  height: 80px;
  width: 80px;
  padding: 10px;
}

.control .outline {
  border: solid fuchsia 5px;
  width: 110px;
  height: 85px;
  padding: 0;
}

.outline.c1 {
  margin: 30px 5px 100px;
}
.outline.c2 {
  padding: 0;
  margin: -20px -45px;
}
.outline.c2 > div {
  margin: -20px 5px 0;
}

.control div {
  margin-left: -35px;
  margin-top: -35px;
}
.control > div {
  margin-left: auto;
  margin-top: 100px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

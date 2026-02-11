# css/css-position/sticky/position-sticky-rendering-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-position/sticky/position-sticky-rendering-ref.html"
}
```

## style[0]

```css

.group {
  display: inline-block;
  position: relative;
  width: 150px;
  height: 250px;
}

.inlineGroup {
  display: inline-block;
  position: relative;
  width: 250px;
  height: 150px;
}

.scroller {
  position: relative;
  width: 100px;
  height: 200px;
  overflow-x: hidden;
  overflow-y: auto;
}

.inlineGroup .scroller {
  position: relative;
  width: 200px;
  height: 100px;
  overflow-x: auto;
  overflow-y: hidden;
}

.contents {
  height: 500px;
}

.inlineGroup .contents {
  height: 100%;
  width: 500px;
}

.indicator {
  background-color: green;
  position: absolute;
}

.box {
  width: 100%;
  height: 100px;
}

.inlineGroup .box {
  height: 100%;
  width: 100px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

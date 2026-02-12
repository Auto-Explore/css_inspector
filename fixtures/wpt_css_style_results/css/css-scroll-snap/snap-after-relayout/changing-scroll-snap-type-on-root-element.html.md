# css/css-scroll-snap/snap-after-relayout/changing-scroll-snap-type-on-root-element.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/snap-after-relayout/changing-scroll-snap-type-on-root-element.html"
}
```

## style[0]

```css

div {
  position: absolute;
  margin: 0;
}

html {
  overflow: hidden;
  scroll-snap-type: none;
}

#y-target {
  width: 300px;
  height: 300px;
  top: 100px;
  left: 0;
  background-color: green;
  scroll-snap-align: start none;
}

#x-target {
  width: 300px;
  height: 300px;
  top: 0;
  left: 100px;
  background-color: red;
  scroll-snap-align: none start;
}

.area {
  width: 1000vw;
  height: 1000vh;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

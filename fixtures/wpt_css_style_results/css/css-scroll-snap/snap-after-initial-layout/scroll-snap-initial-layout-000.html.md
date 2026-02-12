# css/css-scroll-snap/snap-after-initial-layout/scroll-snap-initial-layout-000.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/snap-after-initial-layout/scroll-snap-initial-layout-000.html"
}
```

## style[0]

```css

body {
  position: absolute;
  bottom: 0;
  top: 0;
  left: 0;
  right: 0;
  overflow: hidden;
}

.scroller {
  scroll-snap-type: both mandatory;
  overflow: hidden;
  scroll-padding: 0;
  width: 100px;
  height: 100px;
  border: solid blue;
  margin: 10px;
  display: inline-block;
}

.mandatory > .scroller {
  scroll-snap-type: both mandatory;
}

.proximity > .scroller {
  scroll-snap-type: both proximity;
}

.scroller > div {
  /* padding wrapper */
  width: 30px;
}

.scroller > div > div {
  /* target box */
  height: 30px;
  background: orange;
  scroll-snap-align: start;
}

.proxfar {
  border-color: orange;
}
.proxfar > div > div {
  background: red;
}

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

# css/css-position/sticky/position-sticky-horizontal-rtl-overconstrained.html

```json
{
  "format_version": 3,
  "file": "css/css-position/sticky/position-sticky-horizontal-rtl-overconstrained.html"
}
```

## style[0]

```css


.scroller {
  width: 100px;
  overflow: auto;
  position: relative;
}
.container {
  display: flex;
  position: relative;
  width: 600px;
  direction: rtl;
}
.padding {
  width: 200px;
  height: 100px;
  flex: none;
}
.sticky {
  position: sticky;
  background: green;
  left: 0;
  right: 0;
  width: 200px;
  height: 100px;
  flex: none;
  direction: ltr;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

# css/css-anchor-position/anchor-position-dynamic-003.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-position-dynamic-003.html"
}
```

## style[0]

```css

.containing-block {
  position: absolute;
}
.anchor {
  anchor-name: --a1;
  width: 50px;
  height: 70px;
  background: orange;
}
.after .anchor {
  width: 70px;
  height: 50px;
}
.target {
  position: absolute;
  left: anchor(--a1 right);
  top: anchor(--a1 bottom);
  width: anchor-size(--a1 width);
  height: anchor-size(--a1 height);
  background: green;
}

/* Various types of BFC as the containing block of the anchor */
.float {
  float: left;
}
.table {
  display: table;
}
.inline-block {
  display: inline-block;
  vertical-align: bottom;
}
.contain {
  contain: layout;
}
.scroller {
  overflow: scroll;
  width: 20px;
  height: 20px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

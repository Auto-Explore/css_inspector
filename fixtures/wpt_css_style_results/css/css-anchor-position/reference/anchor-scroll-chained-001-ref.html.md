# css/css-anchor-position/reference/anchor-scroll-chained-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/reference/anchor-scroll-chained-001-ref.html"
}
```

## style[0]

```css

body {
  margin: 0;
}

div {
  width: 100px;
  height: 100px;
}

#scroller {
  overflow: scroll;
}

#anchor {
  height: 20px;
  background: orange;
}

#anchored1 {
  position: absolute;
  top: 50px;
  left: 0;
  background: green;
}

#anchored2 {
  position: absolute;
  top: 150px;
  left: 0;
  background: lime;
}

```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

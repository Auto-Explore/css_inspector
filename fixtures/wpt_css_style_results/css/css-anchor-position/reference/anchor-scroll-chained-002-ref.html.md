# css/css-anchor-position/reference/anchor-scroll-chained-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/reference/anchor-scroll-chained-002-ref.html"
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

#scroller1 {
  width: 200px;
  height: 200px;
}

#scroller1,#scroller2 {
  overflow: scroll;
}

#anchor {
  height: 20px;
  background: orange;
}

#anchored1 {
  position: absolute;
  top: 70px;
  background: green;
}

#anchored2 {
  position: absolute;
  top: 170px;
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

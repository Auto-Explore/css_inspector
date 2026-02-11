# css/css-anchor-position/anchor-scroll-nested.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-scroll-nested.html"
}
```

## style[0]

```css

body {
  margin: 0;
  width: 1500px;
  height: 1500px;
  position: relative;
}

#outer-scroller {
  margin: 500px;
  width: 350px;
  height: 350px;
  outline: 1px solid black;
  overflow: scroll;
}

#inner-scroller {
  margin: 100px;
  width: 250px;
  height: 250px;
  outline: 1px solid black;
  overflow: scroll;
}

#anchor {
  margin: 200px;
  width: 50px;
  height: 50px;
  background-color: green;
  anchor-name: --anchor;
}

.anchored {
  position: absolute;
  width: 50px;
  height: 50px;
  left: anchor(left);
  position-anchor: --anchor;
}

.above {
  bottom: anchor(top);
  background-color: red;
}

.below {
  top: anchor(bottom);
  background-color: yellow;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-anchor”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

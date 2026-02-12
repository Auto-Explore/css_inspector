# css/css-anchor-position/anchor-scroll-overflow-hidden.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-scroll-overflow-hidden.html"
}
```

## style[0]

```css

body {
  font: 20px/1 Ahem;
  margin: 0;
}

#scroll-container {
  width: 400px;
  height: 400px;
  overflow: hidden;
}

#scroll-contents {
  width: 1000px;
  height: 1000px;
}

#placefiller-above-anchor {
  height: 500px;
}

#placefiller-before-anchor {
  display: inline-block;
  width: 500px;
}

#anchor {
  anchor-name: --anchor;
}

#inner-anchored {
  color: green;
  position: absolute;
  left: anchor(left);
  bottom: anchor(top);
  position-anchor: --anchor;
}

#outer-anchored {
  color: yellow;
  position: absolute;
  left: anchor(left);
  top: anchor(bottom);
  position-anchor: --anchor;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

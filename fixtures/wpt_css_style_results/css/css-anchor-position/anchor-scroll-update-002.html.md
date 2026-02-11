# css/css-anchor-position/anchor-scroll-update-002.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-scroll-update-002.html"
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
  overflow: scroll;
}

#scroll-contents {
  width: 1000px;
  height: 1000px;
  position: relative;
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
  left: anchor(--anchor left);
  bottom: anchor(--anchor top);
}

#outer-anchored {
  color: yellow;
  position: absolute;
  left: anchor(--anchor left);
  top: anchor(--anchor bottom);
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

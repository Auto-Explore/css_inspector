# css/css-anchor-position/anchor-scroll-update-001.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-scroll-update-001.html"
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
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-anchor”.",
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

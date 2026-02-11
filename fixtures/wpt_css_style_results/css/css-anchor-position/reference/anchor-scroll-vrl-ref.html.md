# css/css-anchor-position/reference/anchor-scroll-vrl-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/reference/anchor-scroll-vrl-ref.html"
}
```

## style[0]

```css

:root {
  overflow: clip;
}

body {
  font: 20px/1 Ahem;
  margin: 0;
  writing-mode: vertical-rl;
  white-space: nowrap;
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
  width: 480px;
}

#placefiller-before-anchor {
  display: inline-block;
  height: 500px;
}

#anchor {
  anchor-name: --anchor;
}

#inner-anchored {
  margin-top: 520px;
  color: green;
}

#outer-anchored {
  margin-top: 520px;
  color: yellow;
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

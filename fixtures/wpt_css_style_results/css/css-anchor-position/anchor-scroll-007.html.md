# css/css-anchor-position/anchor-scroll-007.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-scroll-007.html"
}
```

## style[0]

```css

body {
  margin: 0;
}

#scroller {
  width: 400px;
  height: 400px;
  overflow: scroll;
  position: relative;
}

#spacer {
  width: 1000px;
  height: 1000px;
}

.anchor {
  width: 50px;
  height: 50px;
  position: absolute;
  background: orange;
}

#anchor1 {
  anchor-name: --a1;
  left: 300px;
  top: 100px;
}

#anchor2 {
  anchor-name: --a2;
  left: 200px;
  top: 200px;
}

#anchor3 {
  anchor-name: --a3;
  left: 100px;
  top: 300px;
}

/* Uses different anchors in insets instead of the default anchor.
 * Still scroll-adjusted because they are in the same scroll container. */
#target {
  position: fixed;
  width: 50px;
  height: 50px;
  left: anchor(--a3 left);
  top: anchor(--a1 top);
  position-anchor: --a2;
  background: lime;
}
```

```json
{
  "errors": 5,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
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

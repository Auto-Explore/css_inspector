# css/css-anchor-position/position-visibility-anchors-visible-clip.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-visibility-anchors-visible-clip.html"
}
```

## style[0]

```css

.clip {
    overflow: hidden;
    border: 2px solid red;
}
.c {
    position: absolute;
    top: 50px;
    width: 100px;
    height: 100px;
    border: 2px solid green;
}
.anchor {
    anchor-name: --foo;
    background: blue;
}
.anchored {
    left: anchor(left);
    top: anchor(bottom);
    position: absolute;
    position-anchor: --foo;
    background: green;
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

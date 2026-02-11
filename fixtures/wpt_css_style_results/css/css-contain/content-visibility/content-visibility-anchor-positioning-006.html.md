# css/css-contain/content-visibility/content-visibility-anchor-positioning-006.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/content-visibility/content-visibility-anchor-positioning-006.html"
}
```

## style[0]

```css

#anchor, #positioned {
  width: 200px;
  height: 200px;
}

#lock {
  height: 400px;
  width: 400px;
  content-visibility: auto;
  border: 1px solid black;
}

pre {
  position: relative;
  top: 200px;
}

#container {
  position:relative;
  height: 10000px;
}

#anchor {
  anchor-name: --anchor;
  background-color: lightblue;
}

#anchor2 {
  anchor-name: --anchor2;
}

#positioned {
  position: absolute;
  background-color: lightgreen;
  left: anchor(right);
  top: anchor( --anchor2 top);
}
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

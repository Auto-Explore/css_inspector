# css/css-contain/content-visibility/content-visibility-anchor-positioning-001.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/content-visibility/content-visibility-anchor-positioning-001.html"
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

#positioned {
  position: absolute;
  background-color: lightgreen;
  left: anchor(right);
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
      "message": "Invalid value for property “background-color”.",
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

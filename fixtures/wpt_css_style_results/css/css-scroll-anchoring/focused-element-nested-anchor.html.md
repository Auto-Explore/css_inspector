# css/css-scroll-anchoring/focused-element-nested-anchor.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-anchoring/focused-element-nested-anchor.html"
}
```

## style[0]

```css


body { height: 4000px; }
div { height: 100px; }

.scroller {
  overflow: scroll;
  position: fixed;
  width: 300px;
  height: 300px;
  background-color: green;
}

#posSticky {
  top: 300px;
  position: relative;
  height: 50px;
  width: 50px;
  background-color: blue;
}

#content {
  background-color: #D3D3D3;
  height: 50px;
  width: 50px;
  position: relative;
  top: 500px;
}

#anchor {
  background-color: brown;
  height: 50px;
  width: 50px;
  position: relative;
  top: 200px;
}

```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

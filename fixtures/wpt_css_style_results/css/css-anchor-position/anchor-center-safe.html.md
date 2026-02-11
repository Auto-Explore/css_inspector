# css/css-anchor-position/anchor-center-safe.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-center-safe.html"
}
```

## style[0]

```css

.anchor {
  position: fixed;
  left: 0px;
  top: 0px;
  height: 30px;
  color: white;
  background-color: green;
  anchor-name: --myAnchor;
}
.infobox {
  color: darkblue;
  background-color: azure;
  border: 1px solid #ddd;
  padding: 10px;
  position: fixed;
  position-anchor: --myAnchor;
  top: calc(anchor(bottom) + 5px);
  left: 0px;
  justify-self: safe anchor-center;
}

.anchor2 {
  position: fixed;
  top: 0px;
  left: 340px;
  height: 30px;
  width: 100px;
  color: white;
  background-color: green;
  anchor-name: --myAnchor2;
}
.infobox2 {
  color: darkblue;
  background-color: azure;
  border: 1px solid #ddd;
  padding: 10px;
  position: fixed;
  position-anchor: --myAnchor2;
  left: calc(anchor(right) + 5px);
  align-self: safe anchor-center;
}
```

```json
{
  "errors": 10,
  "messages": [
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-anchor”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “justify-self”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-anchor”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “align-self”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

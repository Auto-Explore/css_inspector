# css/css-pseudo/relative-box-order-of-pseudo-elements.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/relative-box-order-of-pseudo-elements.html"
}
```

## style[0]

```css

div {
  overflow: auto;
  scroll-marker-group: after;
}

div::before {
  background: red;
  content: "";
  height: 20px;
  width: 100px;
  display: flex;
}

div > li {
  background: orange;
  height: 20px;
  width: 100px;
  display: flex;
}

div::after {
  background: yellow;
  content: "";
  height: 20px;
  width: 100px;
  display: flex;
}

div::scroll-button(up) {
  background: green;
  border: none;
  content: "";
  height: 20px;
  width: 100px;
  display: flex;
}

div::scroll-button(right) {
  content: "";
  border: none;
  background: blue;
  height: 20px;
  width: 100px;
  display: flex;
}


div::scroll-marker-group {
  background: purple;
  height: 20px;
  width: 100px;
  display: flex;
}
```

```json
{
  "errors": 5,
  "messages": [
    {
      "message": "Unknown property “scroll-marker-group”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

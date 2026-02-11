# css/css-anchor-position/anchor-scope-scroll.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-scope-scroll.html"
}
```

## style[0]

```css

#scroller {
  width: 200px;
  height: 200px;
  overflow-y: hidden;
  border: 1px solid black;
}

#item {
  anchor-scope: --item;
  anchor-name: --item;
  height: 50px;
  background: lightgray;
}

#anchored {
  position-anchor: --item;
  position: absolute;
  background: skyblue;
  right: anchor(right);
  top: anchor(top);
  width: 50px;
  height: 50px;
}

.spacer {
  height: 150px;
}
```

```json
{
  "errors": 5,
  "messages": [
    {
      "message": "Unknown property “anchor-scope”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-anchor”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

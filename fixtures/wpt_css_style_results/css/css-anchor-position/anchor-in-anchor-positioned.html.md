# css/css-anchor-position/anchor-in-anchor-positioned.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-in-anchor-positioned.html"
}
```

## style[0]

```css

  .containing-block {
    border: 1px solid black;

    position: relative;
    width: 200px;
    height: 150px;
  }

  .box {
    width: 50px;
    height: 50px;
  }

  #anchor-1 {
    position: absolute;
    top: 50px;
    left: 50px;

    anchor-name: --anchor-1;

    background: green;
  }

  #anchor-positioned-1 {
    position: absolute;
    position-anchor: --anchor-1;
    position-area: top right;
  }

  #anchor-2 {
    anchor-name: --anchor-2;
    background: blue;
  }

  #anchor-positioned-2 {
    position: absolute;
    position-anchor: --anchor-2;
    position-area: bottom right;

    background: cyan;
  }
```

```json
{
  "errors": 7,
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
      "message": "Unknown property “position-area”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-anchor”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-area”.",
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

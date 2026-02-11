# css/css-anchor-position/anchor-css-zoom.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-css-zoom.html"
}
```

## style[0]

```css

    .containing-block {
        position: relative;
        width: 150px;
        height: 150px;

        zoom: 2;
    }

    .cell {
        width: 50px;
        height: 50px;
    }

    #anchor-cell {
        position: absolute;
        top: 50px;
        left: 50px;

        anchor-name: --anchor;

        background: green;
    }

    .anchor-positioned-cell {
        position: absolute;
        position-anchor: --anchor;
    }

    #top-left {
        top: 0;
        right: anchor(left);

        background: cyan;
    }

    #top-right {
        top: 0;
        left: anchor(right);

        background: magenta;
    }

    #bottom-left {
        bottom: 0;
        right: anchor(left);

        background: yellow;
    }

    #bottom-right {
        bottom: 0;
        left: anchor(right);

        background: black;
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
      "message": "Unknown property “position-anchor”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
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

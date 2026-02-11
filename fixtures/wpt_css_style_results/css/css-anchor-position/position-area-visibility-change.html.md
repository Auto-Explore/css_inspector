# css/css-anchor-position/position-area-visibility-change.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-area-visibility-change.html"
}
```

## style[0]

```css

    .containing-block {
      position: relative;
      width: 150px;
      height: 150px;
      outline: 2px black solid;
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

    #target-1 {
      position-area: top right;

      /* Will be changed to 'block' */
      display: none;
    }

    #target-2 {
      position-area: bottom left;

      /* Will be changed to 'visible' */
      visibility: hidden;
    }

    #target-3 {
      position-area: bottom right;

      /* Override default popover style */
      margin: 0;
      padding: 0;
      border: none;
    }

    .blue-background {
      background: blue;
    }

    .magenta-background {
      background: magenta;
    }

    .cyan-background {
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
      "message": "Unknown property “position-area”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-area”.",
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

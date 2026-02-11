# css/css-anchor-position/position-visibility-no-overflow-stacked-child.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-visibility-no-overflow-stacked-child.html"
}
```

## style[0]

```css

  #container {
    position: relative;
    width: 400px;
    height: 150px;
  }

  .anchor {
    width: 100px;
    height: 100px;
    background: orange;
    display: inline-block;
  }

  .target {
    position: absolute;
    position-visibility: no-overflow;
    position-area: block-end;
    width: 100px;
    height: 100px;
    background: red;
    top: 0;
    left: 0;
  }

  #child {
    position: absolute;
    /* stacking context */
    z-index: 1;
    top: -100px;
    left: 100px;
    width: 100px;
    height: 100px;
    background: maroon;
  }

  #grandchild {
    position: absolute;
    z-index: 1;
    top: 0px;
    left: 100px;
    width: 50px;
    height: 50px;
    background: pink;
  }
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-visibility”.",
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

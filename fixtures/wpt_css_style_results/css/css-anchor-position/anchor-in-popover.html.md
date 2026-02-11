# css/css-anchor-position/anchor-in-popover.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-in-popover.html"
}
```

## style[0]

```css

  body {
    margin: 0;
  }

  #containing-block {
    position: relative;
    width: 100px;
    height: 100px;

    border: 1px black solid;
  }

  #anchor-1 {
    width: 50px;
    height: 50px;

    position: absolute;
    top: 25px;
    left: 25px;

    background: green;

    anchor-name: --anchor-1;
  }

  #popover-1 {
    margin: 0;
    border: 1px black solid;
    padding: 0;

    width: 100px;
    height: 100px;

    position: absolute;
    position-anchor: --anchor-1;
    position-area: bottom right;
  }

  #anchor-2 {
    width: 50px;
    height: 50px;

    position: absolute;
    top: 25px;
    left: 25px;

    background: cyan;

    anchor-name: --anchor-2;
  }

  #popover-2 {
    margin: 0;
    border: 1px black solid;
    padding: 0;

    width: 100px;
    height: 100px;

    position: absolute;
    position-anchor: --anchor-2;
    position-area: bottom right;
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
      "message": "Invalid value for property “background”.",
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
    }
  ],
  "warnings": 0
}
```

# css/css-anchor-position/position-area-value.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-area-value.html"
}
```

## style[0]

```css

  #cb {
    position: relative;
    width: 200px;
    height: 200px;
    border: 1px solid black;
  }
  #anchor {
    position: absolute;
    left: 100px;
    top: 100px;
    width: 80px;
    height: 80px;
    background-color: tomato;
    anchor-name: --a;
  }
  #target, #ref {
    position: absolute;
    width: 40px;
    height: 40px;
    background-color: skyblue;
    position-area: bottom right;
    position-anchor: --a;
  }
  #ref {
    background-color: seagreen;
  }
```

```json
{
  "errors": 6,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-area”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-anchor”.",
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

## style[1]

```css

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

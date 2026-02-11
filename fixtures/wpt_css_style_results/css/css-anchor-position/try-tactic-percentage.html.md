# css/css-anchor-position/try-tactic-percentage.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/try-tactic-percentage.html"
}
```

## style[0]

```css

  #cb {
    position: absolute;
    width: 400px;
    height: 400px;
    border: 1px solid black;
  }
  #anchor {
    position: absolute;
    left: 150px;
    top: 150px;
    width: 100px;
    height: 50px;
    background-color: coral;
    anchor-name: --a;
  }
  #target, #ref {
    position: absolute;
    left: 450px; /* force fallback */
    width: 40px;
    height: 40px;
    background-color: skyblue;
    position-anchor: --a;
  }
  #ref {
    background-color: seagreen;
  }
```

```json
{
  "errors": 5,
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

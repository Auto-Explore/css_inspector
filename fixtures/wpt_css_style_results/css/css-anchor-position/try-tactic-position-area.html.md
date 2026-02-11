# css/css-anchor-position/try-tactic-position-area.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/try-tactic-position-area.html"
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
  }
  #target, #ref {
    position: absolute;
    left: 450px; /* force fallback */
    width: 40px;
    height: 40px;
    background-color: skyblue;
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
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

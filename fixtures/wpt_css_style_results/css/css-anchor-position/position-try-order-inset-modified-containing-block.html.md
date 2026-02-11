# css/css-anchor-position/position-try-order-inset-modified-containing-block.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-try-order-inset-modified-containing-block.html"
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
  #target, #ref {
    position: absolute;
    left: 450px; /* force fallback */
    height: 40px;
    background-color: skyblue;
  }
  #ref {
    background-color: seagreen;
  }

  @position-try --margin {
    left:0px;
    right:0px;
    margin: 100px;
  }
  @position-try --no-margin {
    left:0px;
    right:0px;
  }

```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
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

# css/css-anchor-position/base-style-invalidation.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/base-style-invalidation.html"
}
```

## style[0]

```css

  @position-try --pt {
    width: 50px; /* Undo force overflow */
  }
  #cb {
    position: relative;
    width: 200px;
    height: 200px;
    border: 1px solid black;
  }
  #anchor {
    position: absolute;
    left: 75px;
    top: 75px;
    width: 50px;
    height: 50px;
    background: coral;
    anchor-name: --a;
  }
  #anchored {
    position: absolute;
    position-anchor: --a;
    position-try-fallbacks: --pt flip-start;
    inset: 0;
    top: anchor(top);
    bottom: anchor(bottom);
    right: calc(anchor(left) + 5px);
    width: 50px;
    height: 50px;
    background: skyblue;
    justify-self: end;
  }
  #anchored.flip {
    background: seagreen;
    width: 300px; /* Force overflow */
  }
```

```json
{
  "errors": 7,
  "messages": [
    {
      "message": "Unknown at-rule.",
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
      "message": "Unknown property “position-try-fallbacks”.",
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

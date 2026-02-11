# css/css-anchor-position/chained-anchor-scrolled.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/chained-anchor-scrolled.html"
}
```

## style[0]

```css

  #a {
    anchor-name: --b;
    position: fixed;
    position-anchor: --a;
    position-area: bottom;
    background: lime;
  }
  #b {
     position: fixed;
     position-anchor: --b;
     position-area: bottom;
     background: cyan;
  }
  #base {
    anchor-name: --a;
    margin-top: 100vh;
    margin-bottom: 100vh;
    background: lightgrey;
  }
  .box {
    width: 50px;
    height: 50px;
  }
```

```json
{
  "errors": 8,
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
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

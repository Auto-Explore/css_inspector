# css/css-conditional/container-queries/registered-color-style-queries.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/registered-color-style-queries.html"
}
```

## style[0]

```css

  @property --reg-color {
    syntax: "<color>";
    inherits: false;
    initial-value: red;
  }

  #light { color-scheme: light; }
  #dark { color-scheme: dark; }
  .container { --reg-color: light-dark(white, black); }

  @container style(--reg-color: white) {
    #t1 { color: green; }
  }
  @container style(--reg-color: black) {
    #t2 { color: green; }
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

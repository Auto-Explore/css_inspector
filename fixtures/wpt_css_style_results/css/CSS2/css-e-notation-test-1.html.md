# css/CSS2/css-e-notation-test-1.html

```json
{
  "format_version": 3,
  "file": "css/CSS2/css-e-notation-test-1.html"
}
```

## style[0]

```css

    body {background: white; color: black; min-width: 400px}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[1]

```css

  line {stroke: black}
  #p0 {stroke-width: 300px}
  #p1 {stroke-width: 3e2px}
  #p2 {stroke-width: 3.0e2px}
  #p3 {stroke-width: 30.0e1px}
  #p4 {stroke-width: 300.0e0px}
  #p5 {stroke-width: 3e+2px}
  #p6 {stroke-width: 3E+2px}
  #p7 {stroke-width: 0.3E+3px}
  #p8 {stroke-width: 3000.0E-1px}
  #p9 {stroke-width: 30000.000000000000000E-2px}
```

```json
{
  "errors": 11,
  "messages": [
    {
      "message": "Unknown property “stroke”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “stroke-width”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “stroke-width”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “stroke-width”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “stroke-width”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “stroke-width”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “stroke-width”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “stroke-width”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “stroke-width”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “stroke-width”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “stroke-width”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

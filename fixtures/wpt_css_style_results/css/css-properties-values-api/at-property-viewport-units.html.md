# css/css-properties-values-api/at-property-viewport-units.html

```json
{
  "format_version": 3,
  "file": "css/css-properties-values-api/at-property-viewport-units.html"
}
```

## style[0]

```css

  iframe {
    width: 400px;
    height: 200px;
  }
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

    @property --10vw { syntax: '<length>'; inherits: true; initial-value: 10vw}
    @property --10vh { syntax: '<length>'; inherits: true; initial-value: 10vh}
    @property --10vi { syntax: '<length>'; inherits: true; initial-value: 10vi}
    @property --10vb { syntax: '<length>'; inherits: true; initial-value: 10vb}
    @property --10vmin { syntax: '<length>'; inherits: true; initial-value: 10vmin}
    @property --10vmax { syntax: '<length>'; inherits: true; initial-value: 10vmax}

    @property --10svw { syntax: '<length>'; inherits: true; initial-value: 10svw}
    @property --10svh { syntax: '<length>'; inherits: true; initial-value: 10svh}
    @property --10svi { syntax: '<length>'; inherits: true; initial-value: 10svi}
    @property --10svb { syntax: '<length>'; inherits: true; initial-value: 10svb}
    @property --10svmin { syntax: '<length>'; inherits: true; initial-value: 10svmin}
    @property --10svmax { syntax: '<length>'; inherits: true; initial-value: 10svmax}

    @property --10lvw { syntax: '<length>'; inherits: true; initial-value: 10lvw}
    @property --10lvh { syntax: '<length>'; inherits: true; initial-value: 10lvh}
    @property --10lvi { syntax: '<length>'; inherits: true; initial-value: 10lvi}
    @property --10lvb { syntax: '<length>'; inherits: true; initial-value: 10lvb}
    @property --10lvmin { syntax: '<length>'; inherits: true; initial-value: 10lvmin}
    @property --10lvmax { syntax: '<length>'; inherits: true; initial-value: 10lvmax}

    @property --10dvw { syntax: '<length>'; inherits: true; initial-value: 10dvw}
    @property --10dvh { syntax: '<length>'; inherits: true; initial-value: 10dvh}
    @property --10dvi { syntax: '<length>'; inherits: true; initial-value: 10dvi}
    @property --10dvb { syntax: '<length>'; inherits: true; initial-value: 10dvb}
    @property --10dvmin { syntax: '<length>'; inherits: true; initial-value: 10dvmin}
    @property --10dvmax { syntax: '<length>'; inherits: true; initial-value: 10dvmax}
  
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

# css/CSS2/visufx/clip-rect-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/visufx/clip-rect-001.xht"
}
```

## style[0]

```css

    div {
      background: red;
      width: 100px; height: 100px;
    }
    .inner {
      position: absolute;
      background: green;
      clip: rect(0,0,0,0);
      clip: rect( 0 auto auto 0 );
      clip: rect(0,0 0,0);
      clip: rect(0 0,0,0);
      clip: rect(0,0,0 0);
      clip: rect(0 0,0 0);
    }
  
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid value for property “clip”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “clip”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “clip”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “clip”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

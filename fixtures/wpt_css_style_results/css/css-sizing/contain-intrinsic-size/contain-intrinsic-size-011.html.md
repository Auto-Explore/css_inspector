# css/css-sizing/contain-intrinsic-size/contain-intrinsic-size-011.html

```json
{
  "format_version": 3,
  "file": "css/css-sizing/contain-intrinsic-size/contain-intrinsic-size-011.html"
}
```

## style[0]

```css

#flex {
  display: flex;
  flex-direction: column;
  width: 100px;
  height: 200px;
}
.item {
  border: 1px solid black;
}
#one {
  background: lightblue;
  contain-intrinsic-size: 55px 11px;
  contain: size;
  flex-grow: 3;
}
#two {
  background: lightgreen;
  contain-intrinsic-size: 66px 22px;
  contain: size;
  flex-grow: 2;
}
#three {
  background: lightgrey;
  contain-intrinsic-size: 77px 33px;
  contain: size;
  flex-grow: 1;
}
```

```json
{
  "errors": 6,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “contain-intrinsic-size”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “contain-intrinsic-size”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “contain-intrinsic-size”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

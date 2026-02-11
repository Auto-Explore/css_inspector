# css/css-sizing/contain-intrinsic-size/contain-intrinsic-size-021.html

```json
{
  "format_version": 3,
  "file": "css/css-sizing/contain-intrinsic-size/contain-intrinsic-size-021.html"
}
```

## style[0]

```css

#flex {
  display: flex;
  flex-direction: row;
  flex-wrap: wrap;
  align-items: flex-start;
  width: 250px;
}
.item {
  border: 1px solid black;
}
#one {
  background: lightblue;
  contain-intrinsic-size: 55px 21px;
  contain: size;
}
#two {
  background: lightgreen;
  contain-intrinsic-size: 66px 42px;
  contain: size;
}
#three {
  background: lightgrey;
  align-self: stretch;
  contain-intrinsic-size: 77px 63px;
  contain: size;
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

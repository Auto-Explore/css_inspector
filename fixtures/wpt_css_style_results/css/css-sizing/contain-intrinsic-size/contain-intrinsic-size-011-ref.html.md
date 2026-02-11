# css/css-sizing/contain-intrinsic-size/contain-intrinsic-size-011-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-sizing/contain-intrinsic-size/contain-intrinsic-size-011-ref.html"
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
  box-sizing: content-box;
}
#one {
  background: lightblue;
  height: 11px;
  flex-grow: 3;
}
#two {
  background: lightgreen;
  height: 22px;
  flex-grow: 2;
}
#three {
  background: lightgrey;
  height: 33px;
  flex-grow: 1;
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
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

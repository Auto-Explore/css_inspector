# css/css-sizing/contain-intrinsic-size/contain-intrinsic-size-010-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-sizing/contain-intrinsic-size/contain-intrinsic-size-010-ref.html"
}
```

## style[0]

```css

#flex {
  display: flex;
  flex-direction: row;
  width: 500px;
  height: 100px;
}
.item {
  border: 1px solid black;
  box-sizing: content-box;
}
#one {
  background: lightblue;
  width: 55px;
  flex-grow: 3;
}
#two {
  background: lightgreen;
  width: 66px;
  flex-grow: 2;
}
#three {
  background: lightgrey;
  width: 77px;
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

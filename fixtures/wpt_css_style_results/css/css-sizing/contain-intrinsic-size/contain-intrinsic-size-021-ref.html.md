# css/css-sizing/contain-intrinsic-size/contain-intrinsic-size-021-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-sizing/contain-intrinsic-size/contain-intrinsic-size-021-ref.html"
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
  box-sizing: content-box;
}
#one {
  background: lightblue;
  width: 55px;
  height: 21px;
}
#two {
  background: lightgreen;
  width: 66px;
  height: 42px;
}
#three {
  background: lightgrey;
  align-self: stretch;
  width: 77px;
  height: 63px;
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

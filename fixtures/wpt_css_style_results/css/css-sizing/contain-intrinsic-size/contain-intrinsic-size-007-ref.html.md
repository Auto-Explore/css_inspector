# css/css-sizing/contain-intrinsic-size/contain-intrinsic-size-007-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-sizing/contain-intrinsic-size/contain-intrinsic-size-007-ref.html"
}
```

## style[0]

```css

#border {
  border: 1px solid blue;
  width: max-content;
}

#target {
  background: lightblue;
  box-sizing: content-box;
  width: 55px;
  height: 66px;

  border-style: solid;
  border-color: black;
  border-width: 2px 3px 5px 7px;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

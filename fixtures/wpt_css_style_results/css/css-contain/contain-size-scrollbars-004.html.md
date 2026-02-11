# css/css-contain/contain-size-scrollbars-004.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-size-scrollbars-004.html"
}
```

## style[0]

```css

#scroller {
  contain: size;
  background: lightgreen;
  overflow: scroll;
  padding-bottom: 50px;
  width: 100px;
  height: 100px;
}
#content {
  background: lightblue;
  width: 50px;
  height: 130px;
}
```

```json
{
  "errors": 2,
  "messages": [
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

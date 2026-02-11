# css/css-anchor-position/anchor-scope-scroll-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-scope-scroll-ref.html"
}
```

## style[0]

```css

#scroller {
  width: 200px;
  height: 200px;
  border: 1px solid black;
  position: relative;
}

#item {
  height: 50px;
  background: lightgray;
}

#anchored {
  position: absolute;
  background: skyblue;
  right: 0px;
  width: 50px;
  height: 50px;
}

.spacer {
  height: 100px;
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

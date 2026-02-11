# css/CSS2/floats/float-with-absolutely-positioned-child-with-static-inset.html

```json
{
  "format_version": 3,
  "file": "css/CSS2/floats/float-with-absolutely-positioned-child-with-static-inset.html"
}
```

## style[0]

```css

#float {
    float: right;
    width: 250px;
}

#abs-child {
    background: green;
    position: absolute;
    width: 100px;
    height: 100px;
}

#abs-grandchild {
    background: darkgreen;
    position: absolute;
    width: 50px;
    height: 50px;
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

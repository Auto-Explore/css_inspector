# css/css-flexbox/flex-item-contains-size-layout-001.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flex-item-contains-size-layout-001.html"
}
```

## style[0]

```css

#flex {
  display: flex;
  width: 100px;
  height: 100px;
}
#grid {
  background-color: green;
  display: grid;
  flex: 1;
  contain: layout size;
}
.item {
  height: 50px;
  width: 50px;
}
#reference-overlapped-red {
  position: absolute;
  background-color: red;
  width: 100px;
  height: 100px;
  z-index: -1;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “contain”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

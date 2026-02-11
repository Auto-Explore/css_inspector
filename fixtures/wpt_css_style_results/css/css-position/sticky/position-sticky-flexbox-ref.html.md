# css/css-position/sticky/position-sticky-flexbox-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-position/sticky/position-sticky-flexbox-ref.html"
}
```

## style[0]

```css

.scroller {
  overflow: scroll;
  width: 350px;
  height: 100px;
  margin-bottom: 15px;
}

.flex-container {
  width: 600px;
  position: relative;
  display: flex;
  flex-flow: row wrap;
}

.green {
  background-color: green;
}

.flex-item {
  height: 85px;
  width: 100px;
  display: flex;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “flex-flow”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

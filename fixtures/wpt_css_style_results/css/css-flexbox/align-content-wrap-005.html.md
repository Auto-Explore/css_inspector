# css/css-flexbox/align-content-wrap-005.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/align-content-wrap-005.html"
}
```

## style[0]

```css

.flexbox {
  display: flex;
  width: 75px; /* make the row flexbox wrap */
  height: 150px;
  flex-wrap: wrap-reverse;
  position: relative;
  float: left;
  margin-right: 20px;
  border: 1px solid black;
}

.flexitem {
  width: 50px;
  height: 50px;
}

.flexbox :nth-child(1) {
    background-color: blue;
}
.flexbox :nth-child(2) {
    background-color: orange;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

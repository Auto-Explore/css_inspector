# css/css-flexbox/percentage-heights-000.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/percentage-heights-000.html"
}
```

## style[0]

```css

.flexbox {
    display: flex;
    background-color: #aaa;
    position: relative;
}
.flexbox :nth-child(1) {
    background-color: blue;
}
.flexbox :nth-child(2) {
    background-color: green;
}
.flexbox :nth-child(3) {
    background-color: red;
}

.flexbox > div {
    width: 40%;
    height: 40%;
}
.column {
    flex-flow: column wrap;
    width: 100px;
    height: 100px;
    align-content: flex-start;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

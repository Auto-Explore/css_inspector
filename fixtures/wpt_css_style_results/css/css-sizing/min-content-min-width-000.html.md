# css/css-sizing/min-content-min-width-000.html

```json
{
  "format_version": 3,
  "file": "css/css-sizing/min-content-min-width-000.html"
}
```

## style[0]

```css

/* Reduce the heights if you want to add more variants. Don't go less than 2px per stripe. */

.screen {
    width: 100px;
    height: 50px;
    background: red;
}

.container {
    min-width: min-content;
    width: 50px;
    height: 25px;
    background: url(../support/1x1-red.png) repeat-x green;
}
.child {
    width: 100px;
    height: 10px;
    background: green;
    color: transparent;
}

.inline-block .child {
    display: inline-block;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

# css/css-flexbox/flex-flow-013.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flex-flow-013.html"
}
```

## style[0]

```css

body {
    margin: 0;
}
.flexbox {
    width: 600px;
    display: flex;
    background-color: grey;
}
.flexbox > div {
    height: 20px;
    width: 20px;
    border: 0;
}

.rtl {
    direction: rtl;
}

.vertical-rl, .vertical-lr, .column, .column-reverse {
    height: 600px;
}

.vertical-rl {
    writing-mode: vertical-rl;
}

.vertical-lr {
    writing-mode: vertical-lr;
}

.row-reverse {
    flex-flow: row-reverse;
}

.column {
    flex-flow: column;
}

.column-reverse {
    flex-flow: column-reverse;
}

.flexbox > .first {
    background-color: blue;
}
.flexbox > .second {
    background-color: green;
}
.flexbox > .third {
    background-color: red;
}

.flexbox > div > div {
    background-color: orange;
    height: 10px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

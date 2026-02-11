# css/css-overflow/scrollable-overflow-transform-011.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scrollable-overflow-transform-011.html"
}
```

## style[0]

```css

.transformed {
  height: 0;
  width: 0;
  transform: rotate(89deg);
  text-indent: 250px;
}

.transformed::after {
  text-indent: 0;
  content: "";
}

.outer {
  width: 100px;
  height: 100px;
  overflow: hidden;
  background: red;
}

.good {
  width: 200px;
  height: 200px;
  background: green;
}

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

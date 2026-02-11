# css/css-values/calc-size/calc-size-flex-008.html

```json
{
  "format_version": 3,
  "file": "css/css-values/calc-size/calc-size-flex-008.html"
}
```

## style[0]

```css

#flex {
  display: flex;
  flex-direction: column;
  flex-wrap: wrap;

  width: 100px;
  height: calc-size(auto, min(size, 100px));
  background: red;
}

#flex > div {
  width: 100px;
  height: 60px;
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

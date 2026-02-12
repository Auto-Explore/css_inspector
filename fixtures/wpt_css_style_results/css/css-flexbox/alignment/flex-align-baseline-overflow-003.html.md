# css/css-flexbox/alignment/flex-align-baseline-overflow-003.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/alignment/flex-align-baseline-overflow-003.html"
}
```

## style[0]

```css

.target {
  display: flex;
  position: relative;
  line-height: 0;
  font-size: 20px;
  inline-size: 300px;
  margin-block: 10px;
  padding: 10px;
  border: solid 3px;
  writing-mode: vertical-lr;
}

.inner {
  overflow: hidden;
  inline-size: 100px;
  block-size: 80px;
  margin: 10px;
  border: solid 5px;
  padding: 10px;
  font-size: 30px;
}

span {
  display: inline-block;
  width: 1em;
  height: 1em;
  outline: solid cyan 3px;
  outline-offset: -3px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

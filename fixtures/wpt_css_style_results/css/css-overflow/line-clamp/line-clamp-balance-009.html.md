# css/css-overflow/line-clamp/line-clamp-balance-009.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/line-clamp/line-clamp-balance-009.html"
}
```

## style[0]

```css

.clamp {
  font-size: 10px;
  line-clamp: "abcdefghi";
  font-family: monospace;
  width: 49.1ch;
  line-height: 1;
  max-height: 120px;
  text-wrap: balance;
  color: red;
  background: red;

}
b {
    font-weight: normal;
    font-size: 3em;
}
aside {
    position: absolute;
    width: 600px;
    height: 120px;
    background: white;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “line-clamp”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

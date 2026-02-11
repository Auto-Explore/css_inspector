# css/css-overflow/line-clamp/line-clamp-balance-002.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/line-clamp/line-clamp-balance-002.html"
}
```

## style[0]

```css

.clamp {
  line-clamp: 3;
  font-family: monospace;
  width: 8.1ch; /* the extra .1 is just in case the … character isn't perfectly monospaced. */
}
.balance {
    text-wrap: balance;
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

# css/css-anchor-position/anchor-center-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-center-002-ref.html"
}
```

## style[0]

```css

.container {
  width: 100px;
  height: 100px;
  background: orange;
  margin-block: 5px;
}

.item {
  width: 40px;
  height: 40px;
  background: lime;
}

.flex {
  display: flex;
}

.grid {
  display: grid;
  grid-template-columns: repeat(8, 1fr);
  grid-auto-rows: 50px;
  grid-template-areas:
    "a a a a b b b b"
    "a a a a b b b b";
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-template-areas”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

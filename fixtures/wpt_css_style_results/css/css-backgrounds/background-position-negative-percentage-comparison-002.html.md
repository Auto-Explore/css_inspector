# css/css-backgrounds/background-position-negative-percentage-comparison-002.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/background-position-negative-percentage-comparison-002.html"
}
```

## style[0]

```css

.target {
  margin: 50px;
  width: 50px;
  height: 50px;
  border: 1px solid black;
  background-image: url("support/100x100-blue-and-orange.png");
  background-position: min(0%, 100%) max(0%, 100%);
  /* Should behave the same as the declaration above: */
  background-position-x: min(0%, 100%);
  background-position-y: max(0%, 100%);
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

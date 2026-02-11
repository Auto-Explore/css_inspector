# css/css-pseudo/before-as-flex-container.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/before-as-flex-container.html"
}
```

## style[0]

```css

  div {
    width: 200px;
    height: 100px;
    background: red; /* This will show up if the test fails */
  }

  div::before {
    content: "A B"; /* Two "words" to become flex items */
    display: flex;
    justify-content: space-between; /* Pushes "A" and "B" to the edges */
    width: 200px;
    height: 100px;
    background: green; /* This should cover the red background */
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

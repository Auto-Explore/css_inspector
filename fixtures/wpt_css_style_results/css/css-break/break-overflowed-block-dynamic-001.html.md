# css/css-break/break-overflowed-block-dynamic-001.html

```json
{
  "format_version": 3,
  "file": "css/css-break/break-overflowed-block-dynamic-001.html"
}
```

## style[0]

```css

  #multicol {
    columns: 2;
    column-fill: auto;
    column-gap: 0;
    height: 400px; /* Must be large enough so that .target doesn't break initially. */
    width: 100px;
    background: red;
  }
  #block {
    height: 25px;
  }
  #target {
    background: green;
    width: 50px;
    height: 200px; /* Expected to be broken into 2 columns after runTest() */
  }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
